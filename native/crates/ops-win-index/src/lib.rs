#![cfg(windows)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unsafe_code)]
//! Windows file-name index data source (W1 cold MFT enumeration, W4 hard-link
//! reconciliation).
//!
//! This crate is the Windows member of the platform data-source crates designed
//! in #584. W1 delivers the cold half: FSCTL_ENUM_USN_DATA through
//! usn-journal-rs, assembled into ops-index Candidate values with relative
//! paths rebuilt in memory from parent file ids. There is no journal replay,
//! no event stream and no daemon; W2 wires the USN journal and turns a cold
//! enumeration into a caught-up snapshot.
//!
//! The whole crate body is behind #![cfg(windows)], so every other target
//! compiles an empty library and the workspace stays green. The Windows-only
//! dependencies are target-gated for the same reason. usn-journal-rs keeps its
//! unsafe in its own crate; the one exception is W4's hard-link enumeration,
//! where FindFirstFileNameW/FindNextFileNameW have no safe wrapper. That code
//! lives only in `winfs/links.rs`: the crate denies `unsafe_code` everywhere
//! else and allows it on that module alone, and every unsafe block carries a
//! SAFETY comment.
//!
//! What W1 does NOT claim:
//!
//! - No caught-up snapshot. state never answers Ready yet; callers keep
//!   falling back to their own traversal until W2 can replay the journal.
//! - No file size or modification time. FSCTL_ENUM_USN_DATA carries names,
//!   attributes and file ids only, so Candidate.len is 0 and modified_ns is
//!   None until a later enrichment slice fills them.
//! - No volume serial. usn-journal-rs exposes none, so FileKey.volume is 0 and
//!   a key is volume-local for now.
//! - No link application. W4 only enumerates a file's current hard-link names
//!   and classifies an incomplete walk; joining that set to index entries
//!   (W2's HARD_LINK_CHANGE) is the caller's job, and an incomplete walk is
//!   skipped and counted, never applied as an empty set.
//! - No ReFS. A 128-bit file id cannot become the 64-bit half of FileKey, so
//!   an extended id makes this slice answer Unsupported.
//! - No roots below a mounted-volume directory. W1 locates the root by walking
//!   from the volume root through the path components, so the root must live
//!   on a volume mounted at its drive or UNC root.
//!
//! Design reference: goz v0.4.0 (8c6a7b1562bb1eea46dd67d462805b49b9a1f152,
//! MIT). Nothing was copied; this platform layer is written for dsh-ops.

pub mod links;
pub mod mft;
pub mod source;

#[allow(unsafe_code)] // the only Win32 FFI boundary; see winfs/links.rs SAFETY notes.
mod winfs;

pub use links::{
    LinkFailure, LinkFailureKind, LinkOutcome, LinkSet, LinkWalkStats, MAX_LINKS, enumerate,
};
pub use mft::cold_candidates;
pub use source::WinFsSource;

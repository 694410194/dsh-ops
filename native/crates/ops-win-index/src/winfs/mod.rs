// Design informed by mustafaahci/goz (MIT).
//! The crate's only unsafe boundary.
//!
//! Every raw Win32 call is a thin function in this module tree; nothing above
//! it names a handle, a PCWSTR or a Win32 error code. The crate denies
//! unsafe_code globally and allows it on this module alone, so a new unsafe
//! block anywhere else fails the build.

pub mod links;

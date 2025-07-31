#![no_std]
#![cfg_attr(not(feature = "crepr"), forbid(unsafe_code))]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod ansi;
pub use ansi::*;

pub mod csi;
pub use csi::*;

#[cfg(feature = "crepr")]
pub mod ffi;

#[cfg(not(feature = "crepr"))]
pub type MSlice<'a, T> = &'a [T];
#[cfg(feature = "crepr")]
pub type MSlice<'a, T> = ffi::Slice<'a, T>;

#[cfg(not(feature = "crepr"))]
pub type Mchar = core::primitive::char;
#[cfg(feature = "crepr")]
pub type Mchar = u32;

#[cfg(not(feature = "crepr"))]
pub type MOption<T> = Option<T>;
#[cfg(feature = "crepr")]
pub type MOption<T> = ffi::FfiOption<T>;

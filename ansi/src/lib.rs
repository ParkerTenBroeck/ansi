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
pub(crate) type Slice<'a, T> = &'a [T];
#[cfg(feature = "crepr")]
pub type Slice<'a, T> = ffi::Slice<'a, T>;

#[cfg(not(feature = "crepr"))]
pub(crate) type Mchar = core::primitive::char;
#[cfg(feature = "crepr")]
pub type Mchar = u32;

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
#[cfg(feature = "crepr")]
pub use ffi::*;

#[cfg(not(feature = "crepr"))]
pub type FfiSlice<'a, T> = &'a [T];
#[cfg(not(feature = "crepr"))]
pub use Option as FfiOption;
#[cfg(not(feature = "crepr"))]
pub use char as FfiChar;

// // #[cfg(not(feature = "crepr"))]
// // #[cfg(feature = "crepr")]
// pub use ffi::FfiSlice as MSlice;

// // #[cfg(not(feature = "crepr"))]
// // pub use char as Mchar;
// // #[cfg(feature = "crepr")]
// pub use u32 as Mchar;

// // #[cfg(not(feature = "crepr"))]
// // pub use Option as MOption;
// // #[cfg(feature = "crepr")]
// pub use ffi::FfiOption as MOption;

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod ansi;
pub use ansi::*;

pub mod csi;
pub use csi::*;

#[unsafe(no_mangle)]
pub extern "C" fn nya(parser: &mut crate::CSIParser<'static>){

}
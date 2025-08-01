pub mod gr;
pub mod known;
pub mod parser;
#[cfg(test)]
mod test;

pub use gr::*;
pub use known::*;
pub use parser::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(transparent)]
pub struct CSI<'a>(pub crate::FfiSlice<'a, u8>);

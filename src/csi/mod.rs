pub mod gr;
pub mod known;
pub mod parser;
mod test;

pub use gr::*;
pub use known::*;
pub use parser::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CSI<'a> {
    Sequence(CSIParser<'a>),

    SequenceTooLarge,
    IntermediateOverflow,
}

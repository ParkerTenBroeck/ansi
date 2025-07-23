#[cfg(test)]
use crate::{AnsiParser, Out};

#[test]
pub fn csi_sgr_succ() {
    let mut ansi = AnsiParser::<16, 8>::new();
    ansi.parse_csi = false;
    let vals = "\x1b[40m";

    for (i, (c, exp)) in vals
        .chars()
        .zip(
            [
                Out::None,
                Out::None,
                Out::None,
                Out::None,
                Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                    crate::CSI::Unknown {
                        sequence: &[40],
                        intermediate: &[],
                        // modifier: crate::CsiMod::Standard,
                        end: 'm',
                    },
                )))),
            ]
            .into_iter(),
        )
        .enumerate()
    {
        assert_eq!(ansi.next(c), exp, "char index {i}")
    }
}

#[test]
pub fn csi_invalid_ignore() {
    let mut ansi = AnsiParser::<16, 8>::new();
    let vals = "\x1b[1*2*A";

    for (i, (c, exp)) in vals.chars().zip([Out::None; 7].into_iter()).enumerate() {
        assert_eq!(ansi.next(c), exp, "char index {i}")
    }
}

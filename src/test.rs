#[cfg(test)]
use crate::*;

// #[test]
// pub fn csi_sgr_succ() {
//     let mut ansi = AnsiParser::<16, 8>::new();
//     ansi.parse_csi = false;
//     let vals = "\x1b[40m";

//     for (i, (c, exp)) in vals
//         .chars()
//         .zip(
//             [
//                 Out::None,
//                 Out::None,
//                 Out::None,
//                 Out::None,
//                 Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
//                     crate::CSI::Unknown {
//                         sequence: &[40],
//                         intermediate: &[],
//                         // modifier: crate::CsiMod::Standard,
//                         end: 'm',
//                     },
//                 )))),
//             ]
//             .into_iter(),
//         )
//         .enumerate()
//     {
//         assert_eq!(ansi.next(c), exp, "char index {i}")
//     }
// }

#[test]
pub fn utf8() {
    let mut ansi = SizedAnsiParser::<256>::new();
    ansi.bit8_enabled = true;

    let bytes = [0xC3, 0x80, 0x80, 0xC3, 0x00];
    let expected = [
        Out::None,
        Out::Data('À'),
        Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))),
        Out::None,
        Out::InvalidUtf8Sequence,
    ];
    for (i, (b, exp)) in bytes.into_iter().zip(expected.into_iter()).enumerate() {
        assert_eq!(
            ansi.next(b),
            exp,
            "index {i} does not match with expected output"
        )
    }
}

// #[test]
// pub fn csi_invalid_ignore() {
//     let mut ansi = SizedAnsiParser::<256>::new();
//     let vals = "\x1b[1*2*A";

//     for (i, (c, exp)) in vals.chars().zip([Out::None; 7].into_iter()).enumerate() {
//         assert_eq!(ansi.next(c), exp, "char index {i}")
//     }
// }

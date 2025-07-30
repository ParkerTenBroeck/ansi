#[test]
fn csi() {
    let mut parser = crate::SizedAnsiParser::<12>::new();
    parser.max_immediate_count = 4;

    for p in 0x30..=0x3F {
        for i in 0x20..=0x2F {
            for f in 0x40..=0x7E {
                parser.csi_silent_intermediate_overflow = false;
                parser.csi_silent_sequence_overflow = false;

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::Sequence(crate::CSIParser::new(&[p, i, f]))
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::Sequence(crate::CSIParser::new(&[p, f]))
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::Sequence(crate::CSIParser::new(&[i, f]))
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::Sequence(crate::CSIParser::new(&[f]))
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(f), crate::Out::None);

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(f), crate::Out::None);

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::Sequence(crate::CSIParser::new(&[i, i, i, i, f]))
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::IntermediateOverflow
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::IntermediateOverflow
                    ))))
                );

                assert_eq!(parser.next(0x1b), crate::Out::None);
                assert_eq!(parser.next(b'['), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(p), crate::Out::None);
                assert_eq!(parser.next(i), crate::Out::None);
                assert_eq!(
                    parser.next(f),
                    crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
                        crate::CSIResult::SequenceTooLarge
                    ))))
                );
            }
        }
    }

    parser.csi_silent_intermediate_overflow = true;
    parser.csi_silent_sequence_overflow = true;

    assert_eq!(parser.next(0x1b), crate::Out::None);
    assert_eq!(parser.next(b'['), crate::Out::None);
    assert_eq!(parser.next(0x20), crate::Out::None);
    assert_eq!(parser.next(0x21), crate::Out::None);
    assert_eq!(parser.next(0x22), crate::Out::None);
    assert_eq!(parser.next(0x23), crate::Out::None);
    assert_eq!(parser.next(0x24), crate::Out::None);
    assert_eq!(parser.next(0x25), crate::Out::None);
    assert_eq!(
        parser.next(0x40),
        crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
            crate::CSIResult::Sequence(crate::CSIParser::new(&[0x20, 0x21, 0x22, 0x23, 0x40]))
        ))))
    );

    assert_eq!(parser.next(0x1b), crate::Out::None);
    assert_eq!(parser.next(b'['), crate::Out::None);
    for p in 0x30..0x3F {
        assert_eq!(parser.next(p), crate::Out::None);
    }
    assert_eq!(parser.next(0x20), crate::Out::None);
    assert_eq!(parser.next(0x21), crate::Out::None);
    assert_eq!(parser.next(0x22), crate::Out::None);
    assert_eq!(parser.next(0x23), crate::Out::None);
    assert_eq!(parser.next(0x24), crate::Out::None);
    assert_eq!(parser.next(0x25), crate::Out::None);
    assert_eq!(
        parser.next(0x40),
        crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
            crate::CSIResult::Sequence(crate::CSIParser::new(&[
                0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x20, 0x21, 0x22, 0x23, 0x40
            ]))
        ))))
    );

    parser.max_immediate_count = 12;
    assert_eq!(parser.next(0x1b), crate::Out::None);
    assert_eq!(parser.next(b'['), crate::Out::None);
    for p in 0x30..0x3F {
        assert_eq!(parser.next(p), crate::Out::None);
    }
    for i in 0x20..0x2F {
        assert_eq!(parser.next(i), crate::Out::None);
    }
    assert_eq!(
        parser.next(0x40),
        crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
            crate::CSIResult::Sequence(crate::CSIParser::new(&[
                0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2A, 0x40
            ]))
        ))))
    );

    assert_eq!(parser.next(0x1b), crate::Out::None);
    assert_eq!(parser.next(b'['), crate::Out::None);
    for p in (0x30..0x3F).rev() {
        assert_eq!(parser.next(p), crate::Out::None);
    }
    for i in 0x20..0x2F {
        assert_eq!(parser.next(i), crate::Out::None);
    }
    assert_eq!(
        parser.next(0x40),
        crate::Out::Ansi(crate::Ansi::C1(crate::C1::Fe(crate::Fe::CSI(
            crate::CSIResult::Sequence(crate::CSIParser::new(&[
                0x3E, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x40
            ]))
        ))))
    );
}

#[test]
pub fn utf8() {
    use crate::*;

    fn encode(c: u32, vec: &mut std::vec::Vec<u8>) {
        let start = vec.len();
        if c < 0x80 {
            return vec.insert(start, c as u8);
        }

        vec.insert(start, (((c >> 6 * 0) as u8) & 0b111111) | 0b10000000);
        if c < 0x800 {
            return vec.insert(start, ((c >> 6 * 1) as u8 & 0b11111) | 0b11000000);
        }
        vec.insert(start, (((c >> 6 * 1) as u8) & 0b111111) | 0b10000000);
        if c < 10000 {
            return vec.insert(start, ((c >> 6 * 2) as u8 & 0b1111) | 0b11100000);
        }
        vec.insert(start, (((c >> 6 * 2) as u8) & 0b111111) | 0b10000000);
        if c < 200000 {
            return vec.insert(start, ((c >> 6 * 3) as u8 & 0b111) | 0b11110000);
        }
        vec.insert(start, (((c >> 6 * 3) as u8) & 0b111111) | 0b10000000);
        if c < 4000000 {
            return vec.insert(start, ((c >> 6 * 4) as u8 & 0b11) | 0b11111000);
        }
        vec.insert(start, (((c >> 6 * 4) as u8) & 0b111111) | 0b10000000);
        return vec.insert(start, ((c >> 6 * 5) as u8 & 0b1) | 0b11111100);
    }

    let mut ansi = SizedAnsiParser::<256>::new();
    ansi.utf8 = true;
    ansi.del_special = false;
    ansi.space_special = false;

    let mut vec = std::vec::Vec::new();
    for c in 0x32..=u32::MAX >> 1 {
        vec.clear();
        encode(c, &mut vec);

        ansi.bit8_enabled = true;
        assert_eq!(ansi.next(0x00), Out::Ansi(Ansi::C0(C0::NUL)));
        assert_eq!(ansi.next(0x1F), Out::Ansi(Ansi::C0(C0::US)));
        for (i, b) in vec.iter().copied().enumerate() {
            if i == vec.len() - 1 {
                if let Some(c) = char::from_u32(c) {
                    assert_eq!(ansi.next(b), Out::Data(c));
                } else {
                    assert_eq!(ansi.next(b), Out::InvalidCodepoint(c));
                }
            } else {
                assert_eq!(ansi.next(b), Out::None);
            }
        }
        assert_eq!(ansi.next(0x80), Out::Ansi(Ansi::C1(C1::Fe(Fe::PAD))));
        assert_eq!(ansi.next(0x9F), Out::Ansi(Ansi::C1(C1::Fe(Fe::APC))));
        assert_eq!(ansi.next(0x9C), Out::Ansi(Ansi::C1(C1::Fe(Fe::ST))));

        ansi.bit8_enabled = false;
        assert_eq!(ansi.next(0x00), Out::Ansi(Ansi::C0(C0::NUL)));
        assert_eq!(ansi.next(0x1F), Out::Ansi(Ansi::C0(C0::US)));
        for (i, b) in vec.iter().copied().enumerate() {
            if i == vec.len() - 1 {
                if let Some(c) = char::from_u32(c) {
                    assert_eq!(ansi.next(b), Out::Data(c));
                } else {
                    assert_eq!(ansi.next(b), Out::InvalidCodepoint(c));
                }
            } else {
                assert_eq!(ansi.next(b), Out::None);
            }
        }
        assert_eq!(ansi.next(0x80), Out::Data(0x80 as char));
        assert_eq!(ansi.next(0x9E), Out::Data(0x9E as char));
    }
}

#[test]
pub fn invalid_utf8() {
    use crate::*;

    fn invalid_sequence(data: &[u8]) {
        let mut ansi = SizedAnsiParser::<0>::new();
        ansi.utf8 = true;
        for (i, b) in data.iter().copied().enumerate() {
            if i == data.len() - 1 {
                assert_eq!(ansi.next(b), Out::InvalidUtf8Sequence);
            } else {
                assert_eq!(ansi.next(b), Out::None);
            }
        }
    }

    invalid_sequence(&[0b11000000, 0]);

    invalid_sequence(&[0b11100000, 0b10000000, 0]);
    invalid_sequence(&[0b11100000, 0]);

    invalid_sequence(&[0b11110000, 0b10000000, 0b10000000, 0]);
    invalid_sequence(&[0b11110000, 0b10000000, 0]);
    invalid_sequence(&[0b11110000, 0]);

    invalid_sequence(&[0b11111000, 0b10000000, 0b10000000, 0b10000000, 0]);
    invalid_sequence(&[0b11111000, 0b10000000, 0b10000000, 0]);
    invalid_sequence(&[0b11111000, 0b10000000, 0]);
    invalid_sequence(&[0b11111000, 0]);

    invalid_sequence(&[
        0b11111100, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0,
    ]);
    invalid_sequence(&[0b11111100, 0b10000000, 0b10000000, 0b10000000, 0]);
    invalid_sequence(&[0b11111100, 0b10000000, 0b10000000, 0]);
    invalid_sequence(&[0b11111100, 0b10000000, 0]);
    invalid_sequence(&[0b11111100, 0]);
}

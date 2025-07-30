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

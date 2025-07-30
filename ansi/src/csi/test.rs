#[test]
fn csi_parser_empty() {
    let result = crate::CSIParser::new(b"m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [crate::CSIPart::Param(None), crate::CSIPart::Final(b'm')]
    );

    let result = crate::CSIParser::new(b":m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::SubParam(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b"1:m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(Some(1)),
            crate::CSIPart::SubParam(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b":3m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::SubParam(Some(3)),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b":3;m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::SubParam(Some(3)),
            crate::CSIPart::Param(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b":;m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::SubParam(None),
            crate::CSIPart::Param(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b"1:;m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(Some(1)),
            crate::CSIPart::SubParam(None),
            crate::CSIPart::Param(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b";:;m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::Param(None),
            crate::CSIPart::SubParam(None),
            crate::CSIPart::Param(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b"??m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Question,
            crate::CSIPart::Question,
            crate::CSIPart::Param(None),
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b"1?<>=m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(Some(1)),
            crate::CSIPart::Question,
            crate::CSIPart::Lt,
            crate::CSIPart::Gt,
            crate::CSIPart::Eq,
            crate::CSIPart::Final(b'm')
        ]
    );

    let result = crate::CSIParser::new(b"!m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(None),
            crate::CSIPart::Intermediate(b'!'),
            crate::CSIPart::Final(b'm')
        ]
    );
    let result = crate::CSIParser::new(b"44!m").collect::<std::vec::Vec<_>>();
    assert_eq!(
        result,
        [
            crate::CSIPart::Param(Some(44)),
            crate::CSIPart::Intermediate(b'!'),
            crate::CSIPart::Final(b'm')
        ]
    );
}

#[test]
fn csi_parser_intermediate() {
    let mut bytes = *b" m";
    for i in 0x20..=0x2f {
        bytes[0] = i;
        let result = crate::CSIParser::new(&bytes).collect::<std::vec::Vec<_>>();
        assert_eq!(
            result,
            [
                crate::CSIPart::Param(None),
                crate::CSIPart::Intermediate(i),
                crate::CSIPart::Final(b'm')
            ]
        );
    }
}

#[test]
fn csi_parser_final() {
    let mut bytes = *b" ";
    for i in 0x40..=0x7E {
        bytes[0] = i;
        let result = crate::CSIParser::new(&bytes).collect::<std::vec::Vec<_>>();
        assert_eq!(
            result,
            [crate::CSIPart::Param(None), crate::CSIPart::Final(i)]
        );
    }
}

#[test]
fn csi_parser_invalid_byte() {
    let mut bytes = *b" m";
    for i in 0x7F..=0xFE {
        bytes[0] = i;
        let result = crate::CSIParser::new(&bytes).collect::<std::vec::Vec<_>>();
        assert_eq!(result, []);
    }
}

#[cfg(test)]
fn expect_csi(bytes: &[u8], expected: crate::KnownCSI) {
    let mut parser = crate::CSIParser::new(bytes);
    assert_eq!(parser.parse(), expected)
}

#[cfg(test)]
fn expect_csi_params<const N: usize>(
    defaults: [u16; N],
    f: u8,
    func: impl Fn([u16; N]) -> crate::KnownCSI<'static>,
) {
    let mut string = std::string::String::new();
    string.push(f as char);
    for _ in 0..N {
        assert_eq!(
            crate::CSIParser::new(string.as_bytes()).parse(),
            func(defaults),
            "{string}"
        );
        string.insert(0, ';');
    }

    for i in 0..N {
        let mut params = defaults;
        for v in [0, 1, 2, u16::MAX - 1, u16::MAX] {
            params[i] = v;
            string.clear();
            use core::fmt::Write;

            for j in 0..N {
                if i == j {
                    _ = string.write_fmt(format_args!("{}", v));
                }
                if j + 1 != N {
                    string.push(';');
                }
            }
            string.push(f as char);
            assert_eq!(
                crate::CSIParser::new(string.as_bytes()).parse(),
                func(params),
                "{string}"
            );
        }
    }

    let mut params = defaults;
    for v in [0, 1, 2, u16::MAX - 1, u16::MAX] {
        params.fill(v);

        string.clear();
        for (i, param) in params.into_iter().enumerate() {
            use core::fmt::Write;
            _ = string.write_fmt(format_args!("{param}"));
            if i + 1 != N {
                string.push(';');
            }
        }
        string.push(f as char);
        assert_eq!(
            crate::CSIParser::new(string.as_bytes()).parse(),
            func(params),
            "{string}"
        );
    }
}

#[test]
fn private_sequences() {
    let modes = [0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 15, 16, 17, 18, 19];
    for mode in modes {
        let bytes = format!("?{mode}h");
        let mut parser = crate::CSIParser::new(bytes.as_bytes());
        match parser.parse() {
            crate::KnownCSI::ScreenMode(sm) if sm as i32 == mode => {}
            wrong => panic!("{wrong:?}"),
        }
    }

    for mode in modes {
        let bytes = format!("?{mode}l");
        let mut parser = crate::CSIParser::new(bytes.as_bytes());
        match parser.parse() {
            crate::KnownCSI::ResetScreenMode(sm) if sm as i32 == mode => {}
            wrong => panic!("{wrong:?}"),
        }
    }

    expect_csi(b"?25h", crate::KnownCSI::ShowCursor);
    expect_csi(b"?25l", crate::KnownCSI::HideCursor);

    expect_csi(b"?1004h", crate::KnownCSI::EnableFocusReporting);
    expect_csi(b"?1004l", crate::KnownCSI::DisableFocusReporting);

    expect_csi(b"?1049h", crate::KnownCSI::EnableAlternativeBuffer);
    expect_csi(b"?1049l", crate::KnownCSI::DisableAlternativeBuffer);

    expect_csi(b"?2004h", crate::KnownCSI::EnableBracketPastingMode);
    expect_csi(b"?2004l", crate::KnownCSI::DisableBracketPastingMode);
}

#[test]
fn param_val_sequences() {
    expect_csi_params([1], b'A', |[v]| crate::KnownCSI::CursorUp(v));
    expect_csi_params([1], b'B', |[v]| crate::KnownCSI::CursorDown(v));
    expect_csi_params([1], b'C', |[v]| crate::KnownCSI::CursorRight(v));
    expect_csi_params([1], b'D', |[v]| crate::KnownCSI::CursorLeft(v));
    expect_csi_params([1], b'E', |[v]| crate::KnownCSI::CursorNextLine(v));
    expect_csi_params([1], b'F', |[v]| crate::KnownCSI::CursorPreviousLine(v));
    expect_csi_params([1], b'G', |[v]| {
        crate::KnownCSI::CursorHorizontalAbsolute(v)
    });
    expect_csi_params([1, 1], b'H', |[row, col]| crate::KnownCSI::CursorTo {
        row,
        col,
    });

    expect_csi_params([1], b'L', |[v]| crate::KnownCSI::InsertLines(v));
    expect_csi_params([1], b'M', |[v]| crate::KnownCSI::DeleteLines(v));
    expect_csi_params([1], b'S', |[v]| crate::KnownCSI::ScrollUp(v));
    expect_csi_params([1], b'T', |[v]| crate::KnownCSI::ScrollDown(v));

    expect_csi_params([1, 1], b'f', |[row, col]| {
        crate::KnownCSI::HorizontalVerticalPosition { row, col }
    });

    expect_csi_params([1, 1], b'r', |[top, bottom]| {
        crate::KnownCSI::SetScrollingRegion { top, bottom }
    });
    expect_csi_params([], b's', |[]| crate::KnownCSI::SaveCurrentCursorPosition);
    expect_csi_params([], b'u', |[]| crate::KnownCSI::RestoreCurrentCursorPosition);
}

#[test]
fn param_idx_sequences() {
    assert_eq!(
        crate::CSIParser::new(b"J").parse(),
        crate::KnownCSI::EraseDisplay
    );
    assert_eq!(
        crate::CSIParser::new(b"0J").parse(),
        crate::KnownCSI::EraseFromCursor
    );
    assert_eq!(
        crate::CSIParser::new(b"1J").parse(),
        crate::KnownCSI::EraseToCursor
    );
    assert_eq!(
        crate::CSIParser::new(b"2J").parse(),
        crate::KnownCSI::EraseScreen
    );
    assert_eq!(
        crate::CSIParser::new(b"3J").parse(),
        crate::KnownCSI::EraseSavedLines
    );

    assert_eq!(
        crate::CSIParser::new(b"0K").parse(),
        crate::KnownCSI::EraseFromCursorToEndOfLine
    );
    assert_eq!(
        crate::CSIParser::new(b"1K").parse(),
        crate::KnownCSI::EraseStartOfLineToCursor
    );
    assert_eq!(
        crate::CSIParser::new(b"2K").parse(),
        crate::KnownCSI::EraseLine
    );

    assert_eq!(
        crate::CSIParser::new(b"4i").parse(),
        crate::KnownCSI::AuxPortOff
    );
    assert_eq!(
        crate::CSIParser::new(b"5i").parse(),
        crate::KnownCSI::AuxPortOn
    );

    assert_eq!(
        crate::CSIParser::new(b"5n").parse(),
        crate::KnownCSI::DeviceStatusReport
    );
    assert_eq!(
        crate::CSIParser::new(b"6n").parse(),
        crate::KnownCSI::ReportCursorPosition
    );
}

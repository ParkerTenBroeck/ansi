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
fn expect_csi(bytes: &[u8], expected: crate::CSI) {
    let mut parser = crate::CSIParser::new(bytes);
    assert_eq!(parser.parse(), expected)
}

#[cfg(test)]
fn expect_csi_params<const N: usize>(
    defaults: [u16; N],
    f: u8,
    func: impl Fn([u16; N]) -> crate::CSI<'static>,
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
            crate::CSI::ScreenMode(sm) if sm as i32 == mode => {}
            wrong => panic!("{wrong:?}"),
        }
    }

    for mode in modes {
        let bytes = format!("?{mode}l");
        let mut parser = crate::CSIParser::new(bytes.as_bytes());
        match parser.parse() {
            crate::CSI::ResetScreenMode(sm) if sm as i32 == mode => {}
            wrong => panic!("{wrong:?}"),
        }
    }

    expect_csi(b"?25h", crate::CSI::ShowCursor);
    expect_csi(b"?25l", crate::CSI::HideCursor);

    expect_csi(b"?1004h", crate::CSI::EnableFocusReporting);
    expect_csi(b"?1004l", crate::CSI::DisableFocusReporting);

    expect_csi(b"?1049h", crate::CSI::EnableAlternativeBuffer);
    expect_csi(b"?1049l", crate::CSI::DisableAlternativeBuffer);

    expect_csi(b"?2004h", crate::CSI::EnableBracketPastingMode);
    expect_csi(b"?2004l", crate::CSI::DisableBracketPastingMode);
}

#[test]
fn param_val_sequences() {
    expect_csi_params([1], b'A', |[v]| crate::CSI::CursorUp(v));
    expect_csi_params([1], b'B', |[v]| crate::CSI::CursorDown(v));
    expect_csi_params([1], b'C', |[v]| crate::CSI::CursorRight(v));
    expect_csi_params([1], b'D', |[v]| crate::CSI::CursorLeft(v));
    expect_csi_params([1], b'E', |[v]| crate::CSI::CursorNextLine(v));
    expect_csi_params([1], b'F', |[v]| crate::CSI::CursorPreviousLine(v));
    expect_csi_params([1], b'G', |[v]| crate::CSI::CursorHorizontalAbsolute(v));
    expect_csi_params([1, 1], b'H', |[row, col]| crate::CSI::CursorTo { row, col });

    expect_csi_params([1], b'L', |[v]| crate::CSI::InsertLines(v));
    expect_csi_params([1], b'M', |[v]| crate::CSI::DeleteLines(v));
    expect_csi_params([1], b'S', |[v]| crate::CSI::ScrollUp(v));
    expect_csi_params([1], b'T', |[v]| crate::CSI::ScrollDown(v));

    expect_csi_params([1, 1], b'f', |[row, col]| {
        crate::CSI::HorizontalVerticalPosition { row, col }
    });

    expect_csi_params([1, 1], b'r', |[top, bottom]| {
        crate::CSI::SetScrollingRegion { top, bottom }
    });
    expect_csi_params([], b's', |[]| crate::CSI::SaveCurrentCursorPosition);
    expect_csi_params([], b'u', |[]| crate::CSI::RestoreCurrentCursorPosition);
}

#[test]
fn param_idx_sequences() {
    assert_eq!(
        crate::CSIParser::new(b"J").parse(),
        crate::CSI::EraseDisplay
    );
    assert_eq!(
        crate::CSIParser::new(b"0J").parse(),
        crate::CSI::EraseFromCursor
    );
    assert_eq!(
        crate::CSIParser::new(b"1J").parse(),
        crate::CSI::EraseToCursor
    );
    assert_eq!(
        crate::CSIParser::new(b"2J").parse(),
        crate::CSI::EraseScreen
    );
    assert_eq!(
        crate::CSIParser::new(b"3J").parse(),
        crate::CSI::EraseSavedLines
    );

    assert_eq!(
        crate::CSIParser::new(b"0K").parse(),
        crate::CSI::EraseFromCursorToEndOfLine
    );
    assert_eq!(
        crate::CSIParser::new(b"1K").parse(),
        crate::CSI::EraseStartOfLineToCursor
    );
    assert_eq!(crate::CSIParser::new(b"2K").parse(), crate::CSI::EraseLine);

    assert_eq!(crate::CSIParser::new(b"4i").parse(), crate::CSI::AuxPortOff);
    assert_eq!(crate::CSIParser::new(b"5i").parse(), crate::CSI::AuxPortOn);

    assert_eq!(
        crate::CSIParser::new(b"5n").parse(),
        crate::CSI::DeviceStatusReport
    );
    assert_eq!(
        crate::CSIParser::new(b"6n").parse(),
        crate::CSI::ReportCursorPosition
    );
}

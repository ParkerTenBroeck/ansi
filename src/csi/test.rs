#[test]
fn test_csi_parser_empty() {
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
}

#[test]
fn test_csi_parser_intermediate() {
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
fn test_csi_parser_final() {
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
fn test_csi_parser_invalid_byte() {
    let mut bytes = *b" m";
    for i in 0x7F..=0xFF {
        bytes[0] = i;
        let result = crate::CSIParser::new(&bytes).collect::<std::vec::Vec<_>>();
        assert_eq!(result, [crate::CSIPart::Param(None),]);
    }
}

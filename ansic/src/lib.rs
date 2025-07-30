#![no_std]

use ansi::{CSIParser, KnownCSI};

#[repr(C)]
#[derive(Default)]
pub struct AnsiParser {
    pub cfg: ansi::Config,
    state: ansi::ParserState,
}

#[unsafe(no_mangle)]
/// # Safety
/// The parser pointer must be valid.
///
/// The parser must have at least buffer_size bytes allocated after the structure.
pub unsafe extern "C" fn ansic_init(ptr: *mut AnsiParser, buffer_size: usize) {
    unsafe {
        ptr.write(AnsiParser::default());
        ptr.byte_add(core::mem::size_of::<AnsiParser>())
            .write_bytes(0, buffer_size);
    }
}

#[unsafe(no_mangle)]
/// # Safety
/// The parser pointer must be valid and initialized.
///
/// The parser must have at least buffer_size bytes allocated after the structure which are initialized to some value.
///
/// The return value is only valid until any modification is made to the parser.
pub unsafe extern "C" fn ansic_next<'a>(
    parser: *mut AnsiParser,
    buffer_size: usize,
    input: u8,
) -> ansi::Out<'a> {
    let parser = unsafe {
        (core::ptr::slice_from_raw_parts_mut(parser, buffer_size) as *mut ansi::AnsiParser<[u8]>)
            .as_mut()
            .unwrap_unchecked()
    };
    parser.next(input)
}

#[unsafe(no_mangle)]
pub extern "C" fn ansic_parse_csi<'a>(csi: ansi::CSI<'a>) -> KnownCSI<'a> {
    CSIParser::new(csi.0.into()).parse()
}

// #[unsafe(no_mangle)]
// pub extern "C" fn ansic_csi_has_next(parser: &ansi::CSIParser<'_>) -> bool {
//     parser.peek().is_some()
// }

// #[unsafe(no_mangle)]
// /// # Safety
// /// the result of `ansic_csi_has_next` MUST be true for this function to be called
// pub unsafe extern "C" fn ansic_csi_next(parser: &mut ansi::CSIParser<'_>) -> ansi::CSIPart {
//     unsafe { parser.next().unwrap_unchecked() }
// }

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

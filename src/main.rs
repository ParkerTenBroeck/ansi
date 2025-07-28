pub fn main() {
    let mut str = String::new();
    _ = std::io::stdin().read_line(&mut str);
    let mut parser = ansi::SizedAnsiParser::<256>::new();

    for b in str.bytes() {
        dbg!("{:?}", parser.next(b));
    }
}

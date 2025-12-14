use crate::{Attributes, Color, STYLE_LEN, Span, Style};

const FG: Color = Color::BrightCyan;
const BG: Color = Color::DarkGray;
const ATTRS: Attributes = Attributes::all();
const STYLE: Style = Style::new(FG, BG, ATTRS);

#[test]
fn test_descriptor() {
    let desc = STYLE.encode_desc_bytes();

    assert_eq!(desc, [FG.to_byte(), BG.to_byte(), ATTRS.to_byte()])
}

#[test]
fn test_encode() {
    const TEST: &str = "Hello, World 123!";
    const TEST_OUT: &str = "\x01\x20\u{4}\u{1f}Hello, World 123!\x02";

    let mut output = heapless::String::<{ TEST.len() + STYLE_LEN }>::new();

    STYLE.style_to(&mut output, format_args!("{TEST}")).unwrap();

    assert_eq!(output.as_bytes(), TEST_OUT.as_bytes());
}

#[cfg(feature = "alloc")]
#[test]
fn test_encode_heap() {
    const TEST: &str = "Hello, World 123!";
    const TEST_OUT: &str = "\x01\x20\u{4}\u{1f}Hello, World 123!\x02";

    let string = STYLE.style(TEST);

    assert_eq!(string.as_bytes(), TEST_OUT.as_bytes());
}

#[cfg(feature = "alloc")]
#[test]
fn test_decode() {
    const TEST: &str =
        "\x01\x20\u{4}\u{1f}Hello, World 123!\x02\x01\x20\u{4}\u{1f}World, Hello 321?\x02";

    use alloc::string::ToString;

    let out: alloc::vec::Vec<Span> = alloc::vec![
        Span::new("Hello, World 123!".to_string(), STYLE),
        Span::new("World, Hello 321?".to_string(), STYLE),
    ];

    let decoded = Span::decode(TEST).unwrap();

    assert_eq!(decoded, out);
}

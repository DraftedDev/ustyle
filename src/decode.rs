use crate::{Attributes, Color, END, START, Span, Style};

#[cfg(feature = "alloc")]
impl Span {
    /// Decode the given string into one or many spans.
    ///
    /// Only available with the `alloc` feature.
    ///
    /// Returns [None] if parsing failed.
    #[cfg(feature = "alloc")]
    pub fn decode(input: &str) -> Option<alloc::vec::Vec<Span>> {
        let mut spans = alloc::vec::Vec::with_capacity(1);
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut i = 0;

        while i < len {
            // Look for START delimiter
            if bytes[i] != START as u8 {
                i += 1;
                continue;
            }

            // Must have at least [fbaX] structure
            if i + 4 >= len {
                break;
            }

            let fg_byte = bytes[i + 1];
            let bg_byte = bytes[i + 2];
            let attr_byte = bytes[i + 3];

            let mut j = i + 4;
            let mut text_buf = alloc::string::String::new();

            // Read until END delimiter
            while j < len && bytes[j] != END as u8 {
                text_buf.push(bytes[j] as char);
                j += 1;
            }

            // If no closing END, abort parsing
            if j >= len || bytes[j] != END as u8 {
                break;
            }

            spans.push(Span::new(
                text_buf,
                Style::new(
                    Color::parse(fg_byte)?,
                    Color::parse(bg_byte)?,
                    Attributes::parse(attr_byte)?,
                ),
            ));

            // Move past END
            i = j + 1;
        }

        Some(spans)
    }
}

impl Style {
    /// Decode the given bytes into a style.
    ///
    /// Returns [None] if the bytes are invalid.
    pub const fn decode_desc_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 3 {
            return None;
        }

        let foreground = Color::parse(bytes[0]);
        let background = Color::parse(bytes[1]);
        let attributes = Attributes::from_bits(bytes[2]);

        // Use `if let`, because `?` isn't allowed in constant functions.
        if let (Some(foreground), Some(background), Some(attributes)) =
            (foreground, background, attributes)
        {
            Some(Self {
                foreground,
                background,
                attributes,
            })
        } else {
            None
        }
    }

    /// Decode the given string into a style.
    ///
    /// Returns [None] if the string is invalid.
    pub const fn decode_desc(str: &str) -> Option<Self> {
        Self::decode_desc_bytes(str.as_bytes())
    }
}

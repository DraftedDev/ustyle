use crate::{Attributes, Color, Style};

#[cfg(feature = "alloc")]
impl crate::Span {
    /// Decode the given string into one or many spans.
    ///
    /// Only available with the `alloc` feature.
    ///
    /// Returns [None] if parsing failed.
    #[cfg(feature = "alloc")]
    pub fn decode(input: &str) -> Option<alloc::vec::Vec<crate::Span>> {
        Self::decode_capacity(input, 3)
    }

    /// Decode the given string into one or many spans.
    ///
    /// The `capacity` argument is used to pre-allocate the output vector,
    /// so you can tune it to the expected number of spans.
    ///
    /// Only available with the `alloc` feature.
    ///
    /// Returns [None] if parsing failed.
    #[cfg(feature = "alloc")]
    pub fn decode_capacity(input: &str, capacity: usize) -> Option<alloc::vec::Vec<crate::Span>> {
        let bytes = input.as_bytes();
        let mut out = alloc::vec::Vec::with_capacity(capacity);

        let mut i = 0;
        let len = bytes.len();

        while i < len {
            // Find next START
            let Some(rel_start) = memchr::memchr(crate::START as u8, &bytes[i..]) else {
                // Remainder is plain text
                if i < len {
                    out.push(crate::Span {
                        style: Style::default(),
                        text: alloc::string::String::from_utf8(bytes[i..].to_vec()).ok()?,
                    });
                }
                break;
            };

            let start = i + rel_start;

            // Emit plain text before START
            if start > i {
                out.push(crate::Span {
                    style: Style::default(),
                    text: alloc::string::String::from_utf8(bytes[i..start].to_vec()).ok()?,
                });
            }

            // Need at least 3 bytes for f, b, a
            if start + 4 > len {
                return None;
            }

            let text_start = start + 4;

            // Find END
            let rel_end = memchr::memchr(crate::END as u8, &bytes[text_start..])?;

            let text_end = text_start + rel_end;

            out.push(crate::Span {
                style: Style::new(
                    Color::parse(bytes[start + 1])?,
                    Color::parse(bytes[start + 2])?,
                    Attributes::from_bits(bytes[start + 3])?,
                ),
                text: alloc::string::String::from_utf8(bytes[text_start..text_end].to_vec())
                    .ok()?,
            });

            // Advance cursor past END
            i = text_end + 1;
        }

        Some(out)
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

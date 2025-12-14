//! # μStyle || Micro Style
//!
//! **A text-styling library for Rust `no_std` embedded targets.**
//!
//! See the `README.md` for more information.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "alloc")]
extern crate alloc;

/// Decoding functionality for decoding μStyle strings.
pub mod decode;

/// Encoding functionality for encoding μStyle strings.
pub mod encode;

/// Tests for μStyle.
#[cfg(test)]
pub mod tests;

use bitflags::bitflags;

/// Start character for a μStyle string.
pub const START: char = '\x01';

/// End character for a μStyle string.
pub const END: char = '\x02';

/// The length of a style written into bytes.
///
/// Example: `S123Hello WorldE` => 5 style bytes where S is `\x01` and E is `\x02`.
pub const STYLE_LEN: usize = 5;

/// The length of the encoded start tag string.
///
/// Example: `123` => 3 bytes.
pub const DESCRIPTOR_LEN: usize = 3;

/// A span of text coupled with the texts style.
///
/// Only available with the `alloc` feature.
#[cfg(feature = "alloc")]
#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Span {
    /// The text inside the span.
    pub text: alloc::string::String,
    /// The style of the span.
    pub style: Style,
}

#[cfg(feature = "alloc")]
impl Span {
    /// Create a new span with the given text and style.
    pub const fn new(text: alloc::string::String, style: Style) -> Self {
        Self { text, style }
    }

    /// Sets the text of the span and returns itself.
    pub fn with_text(mut self, text: alloc::string::String) -> Self {
        self.text = text;
        self
    }

    /// Sets the style of the span and returns itself.
    pub const fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// The style of a μStyle string.
#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub struct Style {
    /// The foreground color of the text.
    pub foreground: Color,
    /// The background color of the text.
    pub background: Color,
    /// The text attributes. See [Attributes] for more.
    pub attributes: Attributes,
}

impl Style {
    /// Creates a new style.
    pub const fn new(foreground: Color, background: Color, attributes: Attributes) -> Self {
        Self {
            foreground,
            background,
            attributes,
        }
    }

    /// Sets the foreground color of the style and returns itself.
    pub const fn with_foreground(mut self, foreground: Color) -> Self {
        self.foreground = foreground;
        self
    }

    /// Sets the background color of the style and returns itself.
    pub const fn with_background(mut self, background: Color) -> Self {
        self.background = background;
        self
    }

    /// Sets the attributes of the style and returns itself.
    pub const fn with_attribute(mut self, attributes: Attributes) -> Self {
        self.attributes = attributes;
        self
    }
}

/// Colors of μStyle, each assigned a unique byte.
///
/// There is a `None` byte to represent no color (use default color).
///
/// There are 7 base colors:
/// - Gray
/// - Red
/// - Green
/// - Yellow
/// - Blue
/// - Purple
/// - Cyan
///
/// Each base color is divided into 5 variants:
/// - Brighter
/// - Bright
/// - Normal
/// - Dark
/// - Darker.
///
/// Notice that gray replaces black and white, meaning darker gray equals black and brighter gray equals white.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Color {
    /// None - implementations should use the default color
    #[default]
    None = 0,
    /// Gray - rgb(128,128,128)
    Gray = 1,
    /// Bright Gray - rgb(192,192,192)
    BrightGray = 2,
    /// Brighter Gray - rgb(255,255,255)
    BrighterGray = 3,
    /// Dark Gray - rgb(64,64,64)
    DarkGray = 4,
    /// Darker Gray - rgb(0,0,0)
    DarkerGray = 5,
    /// Red - rgb(255,0,0)
    Red = 6,
    /// Bright Red - rgb(255,64,64)
    BrightRed = 7,
    /// Brighter Red - rgb(255,128,128)
    BrighterRed = 8,
    /// Dark Red - rgb(128,0,0)
    DarkRed = 9,
    /// Darker Red - rgb(64,0,0)
    DarkerRed = 10,
    /// Green - rgb(0,255,0)
    Green = 11,
    /// Bright Green - rgb(64,255,64)
    BrightGreen = 12,
    /// Brighter Green - rgb(128,255,128)
    BrighterGreen = 13,
    /// Dark Green - rgb(0,128,0)
    DarkGreen = 14,
    /// Darker Green - rgb(0,64,0)
    DarkerGreen = 15,
    /// Yellow - rgb(255,255,0)
    Yellow = 16,
    /// Bright Yellow - rgb(255,255,64)
    BrightYellow = 17,
    /// Brighter Yellow - rgb(255,255,128)
    BrighterYellow = 18,
    /// Dark Yellow - rgb(128,128,0)
    DarkYellow = 19,
    /// Darker Yellow - rgb(64,64,0)
    DarkerYellow = 20,
    /// Blue - rgb(0,0,255)
    Blue = 21,
    /// Bright Blue - rgb(64,64,255)
    BrightBlue = 22,
    /// Brighter Blue - rgb(128,128,255)
    BrighterBlue = 23,
    /// Dark Blue - rgb(0,0,128)
    DarkBlue = 24,
    /// Darker Blue - rgb(0,0,64)
    DarkerBlue = 25,
    /// Purple - rgb(128,0,128)
    Purple = 26,
    /// Bright Purple - rgb(192,64,192)
    BrightPurple = 27,
    /// Brighter Purple - rgb(224,128,224)
    BrighterPurple = 28,
    /// Dark Purple - rgb(64,0,64)
    DarkPurple = 29,
    /// Darker Purple - rgb(32,0,32)
    DarkerPurple = 30,
    /// Cyan - rgb(0,255,255)
    Cyan = 31,
    /// Bright Cyan - rgb(64,255,255)
    BrightCyan = 32,
    /// Brighter Cyan - rgb(128,255,255)
    BrighterCyan = 33,
    /// Dark Cyan - rgb(0,128,128)
    DarkCyan = 34,
    /// Darker Cyan - rgb(0,64,64)
    DarkerCyan = 35,
}

impl Color {
    /// Parse the given byte to a color.
    ///
    /// Returns [None] if the byte is invalid or represents `None`.
    pub const fn parse(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Color::None),
            1 => Some(Color::Gray),
            2 => Some(Color::BrightGray),
            3 => Some(Color::BrighterGray),
            4 => Some(Color::DarkGray),
            5 => Some(Color::DarkerGray),
            6 => Some(Color::Red),
            7 => Some(Color::BrightRed),
            8 => Some(Color::BrighterRed),
            9 => Some(Color::DarkRed),
            10 => Some(Color::DarkerRed),
            11 => Some(Color::Green),
            12 => Some(Color::BrightGreen),
            13 => Some(Color::BrighterGreen),
            14 => Some(Color::DarkGreen),
            15 => Some(Color::DarkerGreen),
            16 => Some(Color::Yellow),
            17 => Some(Color::BrightYellow),
            18 => Some(Color::BrighterYellow),
            19 => Some(Color::DarkYellow),
            20 => Some(Color::DarkerYellow),
            21 => Some(Color::Blue),
            22 => Some(Color::BrightBlue),
            23 => Some(Color::BrighterBlue),
            24 => Some(Color::DarkBlue),
            25 => Some(Color::DarkerBlue),
            26 => Some(Color::Purple),
            27 => Some(Color::BrightPurple),
            28 => Some(Color::BrighterPurple),
            29 => Some(Color::DarkPurple),
            30 => Some(Color::DarkerPurple),
            31 => Some(Color::Cyan),
            32 => Some(Color::BrightCyan),
            33 => Some(Color::BrighterCyan),
            34 => Some(Color::DarkCyan),
            35 => Some(Color::DarkerCyan),
            _ => None,
        }
    }

    /// Convert this color to a byte.
    pub const fn to_byte(&self) -> u8 {
        *self as u8
    }

    /// Returns the RGB value of this color as a `(r, g, b)` tuple.
    ///
    /// Returns [None] if the color is [Color::None].
    pub const fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        match self {
            Color::None => None,
            Color::Gray => Some((128, 128, 128)),
            Color::BrightGray => Some((192, 192, 192)),
            Color::BrighterGray => Some((255, 255, 255)),
            Color::DarkGray => Some((64, 64, 64)),
            Color::DarkerGray => Some((0, 0, 0)),
            Color::Red => Some((255, 0, 0)),
            Color::BrightRed => Some((255, 64, 64)),
            Color::BrighterRed => Some((255, 128, 128)),
            Color::DarkRed => Some((128, 0, 0)),
            Color::DarkerRed => Some((64, 0, 0)),
            Color::Green => Some((0, 255, 0)),
            Color::BrightGreen => Some((64, 255, 64)),
            Color::BrighterGreen => Some((128, 255, 128)),
            Color::DarkGreen => Some((0, 128, 0)),
            Color::DarkerGreen => Some((0, 64, 0)),
            Color::Yellow => Some((255, 255, 0)),
            Color::BrightYellow => Some((255, 255, 64)),
            Color::BrighterYellow => Some((255, 255, 128)),
            Color::DarkYellow => Some((128, 128, 0)),
            Color::DarkerYellow => Some((64, 64, 0)),
            Color::Blue => Some((0, 0, 255)),
            Color::BrightBlue => Some((64, 64, 255)),
            Color::BrighterBlue => Some((128, 128, 255)),
            Color::DarkBlue => Some((0, 0, 128)),
            Color::DarkerBlue => Some((0, 0, 64)),
            Color::Purple => Some((128, 0, 128)),
            Color::BrightPurple => Some((192, 64, 192)),
            Color::BrighterPurple => Some((224, 128, 224)),
            Color::DarkPurple => Some((64, 0, 64)),
            Color::DarkerPurple => Some((32, 0, 32)),
            Color::Cyan => Some((0, 255, 255)),
            Color::BrightCyan => Some((64, 255, 255)),
            Color::BrighterCyan => Some((128, 255, 255)),
            Color::DarkCyan => Some((0, 128, 128)),
            Color::DarkerCyan => Some((0, 64, 64)),
        }
    }
}

bitflags! {
    /// Text attributes for μStyle, represented as a bitflag.
    ///
    /// There are 5 attributes:
    /// - Bold
    /// - Italic
    /// - Underline
    /// - Strikethrough
    /// - Hidden
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
    pub struct Attributes: u8 {
        /// Make the text bold.
        const BOLD = 1;
        /// Make the text italic.
        const ITALIC = 1 << 1;
        /// Underline the text.
        const UNDERLINE = 1 << 2;
        /// Strikethrough the text.
        const STRIKETHROUGH = 1 << 3;
        /// Hide the text.
        const HIDDEN = 1 << 4;
    }
}

impl Attributes {
    /// Parses the given byte to attributes.
    ///
    /// Returns [None] if the byte is invalid.
    pub const fn parse(byte: u8) -> Option<Self> {
        Self::from_bits(byte)
    }

    /// Convert this attributes to a byte.
    pub const fn to_byte(&self) -> u8 {
        self.bits()
    }
}

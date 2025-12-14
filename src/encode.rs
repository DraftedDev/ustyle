use crate::{DESCRIPTOR_LEN, END, START, STYLE_LEN, Style};
use core::fmt;
use core::fmt::Write;

impl Style {
    /// Encode the style to a 3 byte long descriptor.
    pub fn encode_desc_bytes(&self) -> [u8; DESCRIPTOR_LEN] {
        [
            self.foreground.to_byte(),
            self.background.to_byte(),
            self.attributes.to_byte(),
        ]
    }

    /// Encode the style to a 3 char long descriptor.
    pub fn encode_desc(&self) -> [char; DESCRIPTOR_LEN] {
        [
            char::from(self.foreground.to_byte()),
            char::from(self.background.to_byte()),
            char::from(self.attributes.to_byte()),
        ]
    }

    /// Style the given string by writing the style descriptor and content to the specified writer.
    ///
    /// This will write exactly 5 bytes + the length of the string to the writer.
    pub fn style_to(&self, write: &mut dyn Write, string: &str) -> fmt::Result {
        let [fg, bg, attr] = self.encode_desc();

        write!(write, "{}", START)?;
        write!(write, "{}", fg)?;
        write!(write, "{}", bg)?;
        write!(write, "{}", attr)?;
        write!(write, "{}", string)?;
        write!(write, "{}", END)?;

        Ok(())
    }

    /// Style the given string by writing the start character (`\x01`)
    /// and the style descriptor at the start of the string
    /// and adding the end character (`\x02`) at the end of the string.
    ///
    /// Returns the resulting heap-allocated [alloc::string::String].
    #[cfg(feature = "alloc")]
    pub fn style(&self, string: &str) -> alloc::string::String {
        let mut out = alloc::string::String::with_capacity(string.len() + STYLE_LEN);

        self.style_to(&mut out, string)
            .expect("failed to style string");

        out
    }
}

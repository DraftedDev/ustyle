# μStyle || Micro Style

![Crates.io Version](https://img.shields.io/crates/v/ustyle)
![License](https://img.shields.io/crates/l/ustyle)
![docs.rs](https://img.shields.io/docsrs/ustyle)

**A text-styling library for Rust `no_std` embedded targets.**

## Motivation

After I tried creating a `no_std` embedded ANSI library, I noticed serious flaws in the ANSI format and decided to
create my own [format](FORMAT.md):

μStyle (pronounced "Micro Style") is a ASCII-compatible text-styling [format](FORMAT.md) and library to style text with
colors and
attributes. The
[format](FORMAT.md) is similar to ANSI, but uses way less bytes and is more compact.

This [format](FORMAT.md) allows for fully heap-less styling of text without much stack-overhead.

## Why μStyle?

- Styling only adds 5 bytes of overhead.
- Very predictable behaviour.
- Fast parsing and low overhead.
- Very lightweight (only dependency is `bitflags`).
- Less complex than ANSI.

## Comparison with ANSI

ANSI and μStyle are both ASCII-compatible text-styling formats.

While ANSI uses tags to style text, μStyle uses a fixed descriptor format which only adds **5 bytes** of overhead while
an ANSI tag can add up to **19 bytes** of overhead.

Keep in mind that you need multiple ANSI tags for fully styled strings.

| Feature       | μStyle                             | ANSI                                                                                            |
|---------------|------------------------------------|-------------------------------------------------------------------------------------------------|
| Byte overhead | always 5 bytes                     | up to 19 bytes per tag (possibility of multiple tags, so basically unlimited overhead possible) |
| Colors        | only 35 colors (plus `None` color) | built-in, index or RGB (basically all the colors)                                               |
| Attributes    | bitflag-based and constant sized   | many with separate codes, adding extra style size                                               |

## Limitations

μStyle does have some limitations:

- Only 35 colors (plus `None` color) are supported.
- It's not as expressive as ANSI.
- It's less feature rich than ANSI.
- Delimiters are fixed and cannot be escaped (which is however mostly fine, as they are special characters and not used
  in human-readable text anyway).

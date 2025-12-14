# Format of μStyle

## Syntax

Every styled string starts with a `\x01` and ends with a `\x02`, both being invisible special ASCII characters.

The inner content of the bracket has following format:

`fbaHello World`

where `f` is the foreground color, `b` is the background color and `a` represents the attributes of the string (see
[Descriptor](#descriptor)).

Whatever string is between the start and end markers and after the descriptor is the actual string to be styled.

## Descriptor

The descriptor is a 3 byte long string that represents the actual style of the string.

The first byte is the foreground color, the second byte is the background color and the third byte is the attributes.

## Colors

Every color specified is encoded as **one** character making the color specification as short as possible.

The basic colors included are:

- Gray
- Red
- Green
- Yellow
- Blue
- Purple
- Cyan

Every color is split into 5 categories:

- Primary (basic red, blue, etc.)
- Bright (bright red, bright blue, etc.)
- Brighter (brighter red, brighter blue, etc.)
- Dark (dark red, dark blue, etc.)
- Darker (darker red, darker blue, etc.)

Which makes a total of 35 possible colors plus a `None` color.

### Color Table

| Color                    | Byte |
|--------------------------|------|
| None (use default color) | 0    |
| Gray                     | 1    |
| Bright Gray              | 2    |
| Brighter Gray            | 3    |
| Dark Gray                | 4    |
| Darker Gray              | 5    |
| Red                      | 6    |
| Bright Red               | 7    |
| Brighter Red             | 8    |
| Dark Red                 | 9    |
| Darker Red               | 10   |
| Green                    | 11   |
| Bright Green             | 12   |
| Brighter Green           | 13   |
| Dark Green               | 14   |
| Darker Green             | 15   |
| Yellow                   | 16   |
| Bright Yellow            | 17   |
| Brighter Yellow          | 18   |
| Dark Yellow              | 19   |
| Darker Yellow            | 20   |
| Blue                     | 21   |
| Bright Blue              | 22   |
| Brighter Blue            | 23   |
| Dark Blue                | 24   |
| Darker Blue              | 25   |
| Purple                   | 26   |
| Bright Purple            | 27   |
| Brighter Purple          | 28   |
| Dark Purple              | 29   |
| Darker Purple            | 30   |
| Cyan                     | 31   |
| Bright Cyan              | 32   |
| Brighter Cyan            | 33   |
| Dark Cyan                | 34   |
| Darker Cyan              | 35   |

## Attributes

Attributes are encoded as a single bitflag.

### Attribute Table

| Attribute     | Byte |
|---------------|------|
| Bold          | 1    |
| Italic        | 2    |
| Underline     | 4    |
| Strikethrough | 8    |
| Hidden        | 16   |

use konster::kset::KSet;
use konster::kstr::KStr;
use konster::kvec::KVec;

type CKVec<T> = KVec<T, 50>;
type CKStr = KStr<50>;

fn main() {
    const Q: Template = Template::from_str(r#"{ "foo": "{foo}", "bar": {bar} }"#);
    const R: Template = Template::from_str("{foo:^54.red.on_blue/green.on_cyan}");
    dbg!(Q);
    dbg!(R);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CKSet<T> {
    set: KSet<T, 50>,
}
impl CKSet<Attribute> {
    const fn new() -> Self {
        Self {
            set: KSet {
                vec: KVec {
                    buf: [Attribute::Hidden; 50],
                    cursor: 0,
                },
            },
        }
    }
    #[must_use]
    const fn insert(mut self, attribute: Attribute) -> Self {
        let mut idx = 0;
        while idx < self.set.len() {
            let elem = *self.set.get_by_idx_unchecked(idx);
            if attribute as u8 == elem as u8 {
                self.set.vec.buf[idx] = attribute;
                return self;
            }
            idx += 1;
        }
        self.set.vec.buf[self.set.vec.cursor] = attribute;
        self.set.vec.cursor += 1;
        self
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Color256(u8),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Attribute {
    Bold,
    Dim,
    Italic,
    Underlined,
    Blink,
    Reverse,
    Hidden,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Style {
    fg: Option<Color>,
    bg: Option<Color>,
    fg_bright: bool,
    bg_bright: bool,
    attrs: CKSet<Attribute>,
    force: Option<bool>,
    for_stderr: bool,
}
impl Style {
    const fn new() -> Self {
        Style {
            fg: None,
            bg: None,
            fg_bright: false,
            bg_bright: false,
            attrs: CKSet::new(),
            force: None,
            for_stderr: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Template {
    #[allow(dead_code)]
    parts: TemplatePartVec,
}

impl Template {
    pub const fn from_str(s: &std::primitive::str) -> Self {
        use State::*;
        let (mut state, mut parts, mut buf): (State, TemplatePartVec, CKStr) =
            (Literal, TemplatePartVec::new(), CKStr::new());
        macro_rules! take_buf {
            () => {{
                let res = buf;
                buf = buf.clear();
                res
            }};
        }

        let s = s.as_bytes();
        let mut string_index = 0;
        while string_index < s.len() {
            let c = s[string_index] as char;
            string_index += 1;

            let new = match (state, c) {
                (Literal, '{') => (MaybeOpen, None),
                (Literal, '\n') => {
                    if !buf.is_empty() {
                        parts = parts.push(TemplatePart::Literal(take_buf!()));
                    }
                    parts = parts.push(TemplatePart::NewLine);
                    (Literal, None)
                }
                (Literal, '}') => (DoubleClose, Some('}')),
                (Literal, c) => (Literal, Some(c)),
                (DoubleClose, '}') => (Literal, None),
                (MaybeOpen, '{') => (Literal, Some('{')),
                (MaybeOpen, c) | (Key, c) if c.is_ascii_whitespace() => {
                    // If we find whitespace where the variable key is supposed to go,
                    // backtrack and act as if this was a literal.
                    buf = buf.push(c as u8);
                    let mut new = CKStr::from_str("{");
                    new = new.push_str(&buf);
                    buf = buf.clear();
                    parts = parts.push(TemplatePart::Literal(new));
                    (Literal, None)
                }
                (MaybeOpen, c) if c != '}' && c != ':' => (Key, Some(c)),
                (Key, c) if c != '}' && c != ':' => (Key, Some(c)),
                (Key, ':') => (Align, None),
                (Key, '}') => (Literal, None),
                (Key, '!') if !buf.is_empty() => {
                    parts = parts.push(TemplatePart::Placeholder {
                        key: take_buf!(),
                        align: Alignment::Left,
                        width: None,
                        truncate: true,
                        style: None,
                        alt_style: None,
                    });
                    (Width, None)
                }
                (Align, c) if c == '<' || c == '^' || c == '>' => {
                    if let Some(TemplatePart::Placeholder {
                        align: _align,
                        key,
                        width,
                        truncate,
                        alt_style,
                        style,
                    }) = parts.last_owned()
                    {
                        match c {
                            '<' => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    align: Alignment::Left,
                                    key,
                                    width,
                                    truncate,
                                    alt_style,
                                    style,
                                });
                            }
                            '^' => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    align: Alignment::Center,
                                    key,
                                    width,
                                    truncate,
                                    alt_style,
                                    style,
                                });
                            }
                            '>' => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    align: Alignment::Right,
                                    key,
                                    width,
                                    truncate,
                                    alt_style,
                                    style,
                                });
                            }
                            _ => (),
                        }
                    }

                    (Width, None)
                }
                (Align, c @ '0'..='9') => (Width, Some(c)),
                (Align, '!') | (Width, '!') => {
                    if let Some(TemplatePart::Placeholder {
                        align,
                        key,
                        width,
                        truncate: _truncate,
                        alt_style,
                        style,
                    }) = parts.last_owned()
                    {
                        parts = parts.set_last(TemplatePart::Placeholder {
                            key,
                            align,
                            width,
                            truncate: true,
                            style,
                            alt_style,
                        })
                    }

                    (Width, None)
                }
                (Align, '.') => (FirstStyle, None),
                (Align, '}') => (Literal, None),
                (Width, c @ '0'..='9') => (Width, Some(c)),
                (Width, '.') => (FirstStyle, None),
                (Width, '}') => (Literal, None),
                (FirstStyle, '/') => (AltStyle, None),
                (FirstStyle, '}') => (Literal, None),
                (FirstStyle, c) => (FirstStyle, Some(c)),
                (AltStyle, '}') => (Literal, None),
                (AltStyle, c) => (AltStyle, Some(c)),
                (_st, _c) => panic!("unreachable state"),
            };

            match (state, new.0) {
                (MaybeOpen, Key) if !buf.is_empty() => {
                    parts = parts.push(TemplatePart::Literal(take_buf!()));
                }
                (Key, Align) | (Key, Literal) if !buf.is_empty() => {
                    parts = parts.push(TemplatePart::Placeholder {
                        key: take_buf!(),
                        align: Alignment::Left,
                        width: None,
                        truncate: false,
                        style: None,
                        alt_style: None,
                    });
                }
                (Width, FirstStyle) | (Width, Literal) if !buf.is_empty() => {
                    if let Some(TemplatePart::Placeholder {
                        align,
                        key,
                        width: _width,
                        truncate,
                        alt_style,
                        style,
                    }) = parts.last_owned()
                    {
                        parts = parts.set_last(TemplatePart::Placeholder {
                            key,
                            align,
                            width: Some(buf.parse_usize() as _),
                            truncate,
                            style,
                            alt_style,
                        });
                        buf = buf.clear();
                    }
                }
                (FirstStyle, AltStyle) | (FirstStyle, Literal) if !buf.is_empty() => {
                    if let Some(TemplatePart::Placeholder {
                        align,
                        key,
                        width,
                        truncate,
                        alt_style,
                        style: _style,
                    }) = parts.last_owned()
                    {
                        parts = parts.set_last(TemplatePart::Placeholder {
                            key,
                            align,
                            width,
                            truncate,
                            style: Some(Style::from_dotted_str(&buf)),
                            alt_style,
                        });
                        buf = buf.clear();
                    }
                }
                (AltStyle, Literal) if !buf.is_empty() => {
                    if let Some(TemplatePart::Placeholder {
                        align,
                        key,
                        width,
                        truncate,
                        alt_style: _alt_style,
                        style,
                    }) = parts.last_owned()
                    {
                        parts = parts.set_last(TemplatePart::Placeholder {
                            key,
                            align,
                            width,
                            truncate,
                            style,
                            alt_style: Some(Style::from_dotted_str(&buf)),
                        });
                        buf = buf.clear();
                    }
                }
                (_, _) => (),
            }

            state = new.0;
            if let Some(c) = new.1 {
                buf = buf.push(c as u8);
            }
        }

        if matches!(state, Literal | DoubleClose) && !buf.is_empty() {
            parts = parts.push(TemplatePart::Literal(buf));
        }

        Self { parts }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TemplatePart {
    Literal(CKStr),
    Placeholder {
        key: CKStr,
        align: Alignment,
        width: Option<u16>,
        truncate: bool,
        style: Option<Style>,
        alt_style: Option<Style>,
    },
    NewLine,
}

#[derive(Clone, Copy, PartialEq)]
struct TemplatePartVec(CKVec<TemplatePart>);

impl TemplatePartVec {
    #[must_use]
    const fn new() -> Self {
        Self(CKVec {
            buf: [TemplatePart::NewLine; 50],
            cursor: 0,
        })
    }
    #[must_use]
    const fn push(mut self, part: TemplatePart) -> Self {
        self.0.buf[self.0.cursor] = part;
        self.0.cursor += 1;
        self
    }
    #[must_use]
    const fn set_last(mut self, style: TemplatePart) -> Self {
        if self.0.is_empty() {
            panic!("CKVec is empty");
        }
        self.0.buf[self.0.cursor - 1] = style;
        self
    }
    pub const fn last_owned(&self) -> Option<TemplatePart> {
        if self.0.is_empty() {
            return None;
        }
        Some(self.0.buf[self.0.cursor - 1])
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Literal,
    MaybeOpen,
    DoubleClose,
    Key,
    Align,
    Width,
    FirstStyle,
    AltStyle,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Alignment {
    Left,
    Center,
    Right,
}

impl std::fmt::Debug for TemplatePartVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CKVec")
            .field("inner", &&self.0.buf[..self.0.cursor])
            .finish()
    }
}

impl Style {
    pub const fn from_dotted_str(s: &CKStr) -> Style {
        let mut rv = Style::new();
        let mut idx = 0;
        let elems = s.split::<20, 20>(b'.');
        while idx < elems.len() {
            let part = elems.get_unchecked(idx);
            idx += 1;

            let mut buf = [0; 20];
            let mut buf_idx = 0;
            while buf_idx < part.len() {
                buf[buf_idx] = *part.get_unchecked(buf_idx);
                buf_idx += 1;
            }
            let mut buf = buf.as_slice();
            // Remove trailing zeroes
            loop {
                match buf.split_last() {
                    Some((a, b)) => {
                        if *a == 0 {
                            buf = b;
                            continue;
                        }
                    }
                    _ => (),
                };
                break;
            }

            rv = match buf {
                b"black" => rv.black(),
                b"red" => rv.red(),
                b"green" => rv.green(),
                b"yellow" => rv.yellow(),
                b"blue" => rv.blue(),
                b"magenta" => rv.magenta(),
                b"cyan" => rv.cyan(),
                b"white" => rv.white(),
                b"bright" => rv.bright(),
                b"on_black" => rv.on_black(),
                b"on_red" => rv.on_red(),
                b"on_green" => rv.on_green(),
                b"on_yellow" => rv.on_yellow(),
                b"on_blue" => rv.on_blue(),
                b"on_magenta" => rv.on_magenta(),
                b"on_cyan" => rv.on_cyan(),
                b"on_white" => rv.on_white(),
                b"on_bright" => rv.on_bright(),
                b"bold" => rv.bold(),
                b"dim" => rv.dim(),
                b"underlined" => rv.underlined(),
                b"blink" => rv.blink(),
                b"reverse" => rv.reverse(),
                b"hidden" => rv.hidden(),
                on_c if starts_with(on_c, "on_".as_bytes()) => {
                    if let Ok(n) = parse_u8(on_c) {
                        rv.on_color256(n)
                    } else {
                        continue;
                    }
                }
                c => {
                    if let Ok(n) = parse_u8(c) {
                        rv.color256(n)
                    } else {
                        continue;
                    }
                }
            };
        }
        rv
    }

    /// Sets a foreground color.
    #[inline]
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Sets a background color.
    #[inline]
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Adds a attr.
    #[inline]
    pub const fn attr(mut self, attr: Attribute) -> Style {
        self.attrs = self.attrs.insert(attr);
        self
    }

    #[inline]
    pub const fn black(self) -> Style {
        self.fg(Color::Black)
    }
    #[inline]
    pub const fn red(self) -> Style {
        self.fg(Color::Red)
    }
    #[inline]
    pub const fn green(self) -> Style {
        self.fg(Color::Green)
    }
    #[inline]
    pub const fn yellow(self) -> Style {
        self.fg(Color::Yellow)
    }
    #[inline]
    pub const fn blue(self) -> Style {
        self.fg(Color::Blue)
    }
    #[inline]
    pub const fn magenta(self) -> Style {
        self.fg(Color::Magenta)
    }
    #[inline]
    pub const fn cyan(self) -> Style {
        self.fg(Color::Cyan)
    }
    #[inline]
    pub const fn white(self) -> Style {
        self.fg(Color::White)
    }
    #[inline]
    pub const fn color256(self, color: u8) -> Style {
        self.fg(Color::Color256(color))
    }

    #[inline]
    pub const fn bright(mut self) -> Style {
        self.fg_bright = true;
        self
    }

    #[inline]
    pub const fn on_black(self) -> Style {
        self.bg(Color::Black)
    }
    #[inline]
    pub const fn on_red(self) -> Style {
        self.bg(Color::Red)
    }
    #[inline]
    pub const fn on_green(self) -> Style {
        self.bg(Color::Green)
    }
    #[inline]
    pub const fn on_yellow(self) -> Style {
        self.bg(Color::Yellow)
    }
    #[inline]
    pub const fn on_blue(self) -> Style {
        self.bg(Color::Blue)
    }
    #[inline]
    pub const fn on_magenta(self) -> Style {
        self.bg(Color::Magenta)
    }
    #[inline]
    pub const fn on_cyan(self) -> Style {
        self.bg(Color::Cyan)
    }
    #[inline]
    pub const fn on_white(self) -> Style {
        self.bg(Color::White)
    }
    #[inline]
    pub const fn on_color256(self, color: u8) -> Style {
        self.bg(Color::Color256(color))
    }

    #[inline]
    pub const fn on_bright(mut self) -> Style {
        self.bg_bright = true;
        self
    }

    #[inline]
    pub const fn bold(self) -> Style {
        self.attr(Attribute::Bold)
    }
    #[inline]
    pub const fn dim(self) -> Style {
        self.attr(Attribute::Dim)
    }
    #[inline]
    pub const fn italic(self) -> Style {
        self.attr(Attribute::Italic)
    }
    #[inline]
    pub const fn underlined(self) -> Style {
        self.attr(Attribute::Underlined)
    }
    #[inline]
    pub const fn blink(self) -> Style {
        self.attr(Attribute::Blink)
    }
    #[inline]
    pub const fn reverse(self) -> Style {
        self.attr(Attribute::Reverse)
    }
    #[inline]
    pub const fn hidden(self) -> Style {
        self.attr(Attribute::Hidden)
    }
}
const fn starts_with(b: &[u8], pat: &[u8]) -> bool {
    let mut idx = 0;
    while idx < pat.len() {
        if b[idx] != pat[idx] {
            return false;
        }
        idx += 1;
    }
    true
}
const fn parse_u8(b: &[u8]) -> Result<u8, ()> {
    let mut idx = 0;
    let mut pow = b.len() - 1;
    let mut result = 0;
    while idx < b.len() {
        result += (b[idx] as usize - 48) * 10_usize.pow(pow as _);
        if pow == 0 {
            break;
        }
        pow -= 1;
        idx += 1;
    }
    if result < 256 {
        Ok(result as u8)
    } else {
        Err(())
    }
}

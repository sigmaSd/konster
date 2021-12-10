use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::mem;

fn main() {}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Style;

type String = KVec<u8>;
type str = KVec<u8>;

#[derive(Clone, Copy, PartialEq)]
struct KVec<T> {
    a: [T; 50],
    idx: usize,
}
impl std::fmt::Debug for String {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl std::fmt::Debug for KVec<TemplatePart> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KVec")
            .field("inner", &&self.a[..self.idx])
            .finish()
    }
}
impl KVec<TemplatePart> {
    const fn new() -> Self {
        Self {
            a: [TemplatePart::NewLine; 50],
            idx: 0,
        }
    }
    const fn last(&self) -> Option<TemplatePart> {
        if self.is_empty() {
            return None;
        }
        Some(self.a[self.idx - 1])
    }
    const fn push(mut self, s: TemplatePart) -> Self {
        self.a[self.idx] = s;
        self.idx += 1;
        self
    }
    const fn set_last(mut self, style: TemplatePart) -> Self {
        if self.is_empty() {
            panic!("KVec is empty");
        }
        self.a[self.idx - 1] = style;
        self
    }
}
impl String {
    const fn new() -> Self {
        Self { a: [0; 50], idx: 0 }
    }
    const fn push_str(mut self, s: &Self) -> Self {
        let mut q = 0;
        while q < s.len() {
            self = self.push(s.a[q]);
            q += 1;
        }
        self
    }
    const fn from(s: &std::primitive::str) -> Self {
        let s = s.as_bytes();
        let mut a = [0; 50];
        let mut q = 0;
        while q < s.len() {
            a[q] = s[q];
            q += 1;
        }
        Self { a, idx: 0 }
    }
    #[must_use]
    const fn push(mut self, s: u8) -> Self {
        self.a[self.idx] = s;
        self.idx += 1;
        self
    }
    const fn parse(self) -> u16 {
        let mut q = 0;
        let mut r = 0;
        let mut pow = self.idx - 1;

        loop {
            r += (self.a[q] as usize - 48) * 10_usize.pow(pow as _);
            if pow == 0 {
                break;
            }
            pow -= 1;
            q += 1;
        }
        r as _
    }
    fn as_str(&self) -> &std::primitive::str {
        std::str::from_utf8(&self.a[..self.idx]).unwrap()
    }
}
impl<T> KVec<T> {
    const fn len(&self) -> usize {
        self.idx
    }
    const fn is_empty(&self) -> bool {
        const Q: Template = Template::from_str(r#"{ "foo": "{foo}", "bar": {bar} }"#);
        const R: Template = Template::from_str("{foo:^54.red.on_blue/green.on_cyan}");
        #[test]
        fn a() {
            dbg!(Q);
            dbg!(R);
        }
        self.idx == 0
    }
    #[must_use]
    const fn clear(mut self) -> Self {
        self.idx = 0;
        self
    }
}

#[derive(Clone, Debug)]
struct Template {
    parts: KVec<TemplatePart>,
}

impl Template {
    const fn from_str(s: &std::primitive::str) -> Self {
        use State::*;
        let (mut state, mut parts, mut buf): (State, KVec<TemplatePart>, String) =
            (Literal, KVec::<TemplatePart>::new(), String::new());
        macro_rules! take_buf {
            () => {{
                let res = buf;
                buf = buf.clear();
                res
            }};
        }

        let s = s.as_bytes();
        let mut qsd = 0;
        while qsd < s.len() {
            let c = s[qsd] as char;
            qsd += 1;

            let new = match (state, c) {
                (Literal, '{') => (MaybeOpen, None),
                (Literal, '\n') => {
                    if !buf.is_empty() {
                        parts = parts.push(TemplatePart::Literal(take_buf!()));
                    }
                    parts.push(TemplatePart::NewLine);
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
                    let mut new = String::from("{");
                    new.push_str(&buf);
                    buf = buf.clear();
                    parts.push(TemplatePart::Literal(new));
                    (Literal, None)
                }
                (MaybeOpen, c) if c != '}' && c != ':' => (Key, Some(c)),
                (Key, c) if c != '}' && c != ':' => (Key, Some(c)),
                (Key, ':') => (Align, None),
                (Key, '}') => (Literal, None),
                (Key, '!') if !buf.is_empty() => {
                    parts.push(TemplatePart::Placeholder {
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
                    if !parts.is_empty() {
                        match parts.last() {
                            Some(TemplatePart::Placeholder {
                                align,
                                key,
                                width,
                                truncate,
                                alt_style,
                                style,
                            }) => match c {
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
                            },
                            _ => (),
                        }
                    }

                    (Width, None)
                }
                (Align, c @ '0'..='9') => (Width, Some(c)),
                (Align, '!') | (Width, '!') => {
                    if !parts.is_empty() {
                        match parts.last() {
                            Some(TemplatePart::Placeholder {
                                align,
                                key,
                                width,
                                truncate,
                                alt_style,
                                style,
                            }) => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    key,
                                    align,
                                    width,
                                    truncate: true,
                                    style,
                                    alt_style,
                                })
                            }
                            _ => (),
                        }
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
                (st, c) => panic!("unreachable state"),
            };

            match (state, new.0) {
                (MaybeOpen, Key) if !buf.is_empty() => {
                    parts = parts.push(TemplatePart::Literal(take_buf!()));
                    buf = take_buf!();
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
                    if !parts.is_empty() {
                        match parts.last() {
                            Some(TemplatePart::Placeholder {
                                align,
                                key,
                                width,
                                truncate,
                                alt_style,
                                style,
                            }) => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    key,
                                    align,
                                    width: Some(buf.parse()),
                                    truncate,
                                    style,
                                    alt_style,
                                });
                                buf = buf.clear();
                            }
                            _ => (),
                        }
                    }
                }
                (FirstStyle, AltStyle) | (FirstStyle, Literal) if !buf.is_empty() => {
                    if !parts.is_empty() {
                        match parts.last() {
                            Some(TemplatePart::Placeholder {
                                align,
                                key,
                                width,
                                truncate,
                                alt_style,
                                style,
                            }) => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    key,
                                    align,
                                    width,
                                    truncate,
                                    style: Some(Style),
                                    alt_style,
                                });
                                buf = buf.clear();
                            }
                            _ => (),
                        }
                    }
                }
                (AltStyle, Literal) if !buf.is_empty() => {
                    if !parts.is_empty() {
                        match parts.last() {
                            Some(TemplatePart::Placeholder {
                                align,
                                key,
                                width,
                                truncate,
                                alt_style,
                                style,
                            }) => {
                                parts = parts.set_last(TemplatePart::Placeholder {
                                    key,
                                    align,
                                    width,
                                    truncate,
                                    style,
                                    alt_style: Some(Style),
                                });
                                buf = buf.clear();
                            }
                            _ => (),
                        }
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
            parts.push(TemplatePart::Literal(buf));
        }

        Self { parts }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TemplatePart {
    Literal(String),
    Placeholder {
        key: String,
        align: Alignment,
        width: Option<u16>,
        truncate: bool,
        style: Option<Style>,
        alt_style: Option<Style>,
    },
    NewLine,
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

const Q: Template = Template::from_str(r#"{ "foo": "{foo}", "bar": {bar} }"#);
const R: Template = Template::from_str("{foo:^54.red.on_blue/green.on_cyan}");
#[test]
fn a() {
    dbg!(Q);
    dbg!(R);
}

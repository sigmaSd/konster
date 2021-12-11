use konster::{kstr::KStr, kvec::KVec};

fn main() {
    const Q: Template = Template::from_str(r#"{ "foo": "{foo}", "bar": {bar} }"#);
    const R: Template = Template::from_str("{foo:^54.red.on_blue/green.on_cyan}");
    dbg!(Q);
    dbg!(R);
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Style;

#[derive(Clone, Debug)]
pub struct Template {
    #[allow(dead_code)]
    parts: TemplatePartVec,
}

impl Template {
    pub const fn from_str(s: &std::primitive::str) -> Self {
        use State::*;
        let (mut state, mut parts, mut buf): (State, TemplatePartVec, KStr) =
            (Literal, TemplatePartVec::new(), KStr::new());
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
                    let mut new = KStr::from_str("{");
                    new = new.push_kstr(&buf);
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
                            width: Some(buf.parse()),
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
                            style: Some(Style),
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
                            alt_style: Some(Style),
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
    Literal(KStr),
    Placeholder {
        key: KStr,
        align: Alignment,
        width: Option<u16>,
        truncate: bool,
        style: Option<Style>,
        alt_style: Option<Style>,
    },
    NewLine,
}

#[derive(Clone, Copy, PartialEq)]
struct TemplatePartVec(KVec<TemplatePart>);

impl TemplatePartVec {
    #[must_use]
    const fn new() -> Self {
        Self(KVec {
            buf: [TemplatePart::NewLine; 50],
            idx: 0,
        })
    }
    #[must_use]
    const fn push(mut self, part: TemplatePart) -> Self {
        self.0.buf[self.0.idx] = part;
        self.0.idx += 1;
        self
    }
    #[must_use]
    const fn set_last(mut self, style: TemplatePart) -> Self {
        if self.0.is_empty() {
            panic!("KVec is empty");
        }
        self.0.buf[self.0.idx - 1] = style;
        self
    }
    pub const fn last_owned(&self) -> Option<TemplatePart> {
        if self.0.is_empty() {
            return None;
        }
        Some(self.0.buf[self.0.idx - 1])
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
        f.debug_struct("KVec")
            .field("inner", &&self.0.buf[..self.0.idx])
            .finish()
    }
}

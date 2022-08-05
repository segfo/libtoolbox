//
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UnicodeErrorKind {
    IllegalByteSequence,
    RedundantEncoding,
    IllegalCodePoint,
    IllegalRange,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct UnicodeParseError {
    kind: UnicodeErrorKind,
}

impl UnicodeParseError {
    pub fn new(kind: UnicodeErrorKind) -> Self {
        UnicodeParseError { kind: kind }
    }
}

impl std::fmt::Display for UnicodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ["Invalid utf-8 byte sequence.",
        "Redundant encoding.",
        "It could not be expressed in utf-8 because the code-point is invalid. It could be pointing to the invalid code point of a surrogate pair, etc.",
        "Could not be expressed in utf-8 because the range of the code-point is invalid. It currently supports up to the 16th side of the code-space."
        ][self.kind as usize])
    }
}

impl std::error::Error for UnicodeParseError {}

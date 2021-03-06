use std::fmt::Display;

use super::diagnostic_kind::DiagnosticKind;
use super::text::text_span::TextSpan;

#[derive(Debug)]
pub struct Diagnostic {
    span: TextSpan,
    message: String,
    kind: DiagnosticKind,
}

impl Diagnostic {
    pub fn new(span: TextSpan, message: String, kind: DiagnosticKind) -> Self {
        Self {
            span,
            message,
            kind,
        }
    }

    pub fn span(&self) -> TextSpan {
        self.span.clone()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn kind(&self) -> &DiagnosticKind {
        &self.kind
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

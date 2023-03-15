mod compiler;
mod engine;
mod feedback;
mod files;
mod progress;
mod protocol_adapter;

use lsp_types::{Position, Range};
use std::any::Any;

// TODO: remove all these re-exports
pub use compiler::LspProjectCompiler;
pub use engine::{LanguageServerEngine, Response};
pub use feedback::{Feedback, FeedbackBookKeeper};
pub use files::FileSystemProxy;
pub use progress::ProgressReporter;
pub use protocol_adapter::LanguageServerProtocolAdapter;

use crate::{ast::SrcSpan, line_numbers::LineNumbers};

#[derive(Debug)]
pub struct LockGuard(pub Box<dyn Any>);

pub trait Locker {
    fn lock_for_build(&self) -> LockGuard;
}

pub fn src_span_to_lsp_range(location: SrcSpan, line_numbers: &LineNumbers) -> Range {
    let start = line_numbers.line_and_column_number(location.start);
    let end = line_numbers.line_and_column_number(location.end);

    Range::new(
        Position::new(start.line - 1, start.column - 1),
        Position::new(end.line - 1, end.column - 1),
    )
}
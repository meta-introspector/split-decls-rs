use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Dummy things for testing where spans don't matter.
pub mod dummy_test_span_utils {
    use span::{Span, SyntaxContext};
    use super::*;
    pub const DUMMY: Span = Span {
        range: TextRange::empty(TextSize::new(0)),
        anchor: span::SpanAnchor {
            file_id: span::EditionedFileId::new(
                span::FileId::from_raw(0xe4e4e),
                span::Edition::CURRENT,
            ),
            ast_id: span::ROOT_ERASED_FILE_AST_ID,
        },
        ctx: SyntaxContext::root(Edition::CURRENT),
    };
    pub struct DummyTestSpanMap;
    impl SpanMapper<Span> for DummyTestSpanMap {
        fn span_for(&self, range: syntax::TextRange) -> Span {
            Span {
                range,
                anchor: span::SpanAnchor {
                    file_id: span::EditionedFileId::new(
                        span::FileId::from_raw(0xe4e4e),
                        span::Edition::CURRENT,
                    ),
                    ast_id: span::ROOT_ERASED_FILE_AST_ID,
                },
                ctx: SyntaxContext::root(Edition::CURRENT),
            }
        }
    }
}

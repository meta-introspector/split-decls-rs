use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<Span: Copy> DelimSpan<Span> {
    pub fn from_single(sp: Span) -> Self {
        DelimSpan {
            open: sp,
            close: sp,
        }
    }
    pub fn from_pair(open: Span, close: Span) -> Self {
        DelimSpan { open, close }
    }
}

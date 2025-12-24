use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl MetavarSpansMap {
    pub fn insert(&self, span: Span, var_span: Span) -> bool {
        match self.0.write().try_insert(span, (var_span, false)) {
            Ok(_) => true,
            Err(entry) => entry.entry.get().0 == var_span,
        }
    }
    /// Read a span and record that it was read.
    pub fn get(&self, span: Span) -> Option<Span> {
        if let Some(mut mspans) = self.0.try_write() {
            if let Some((var_span, read)) = mspans.get_mut(&span) {
                *read = true;
                Some(*var_span)
            } else {
                None
            }
        } else {
            if let Some((span, true)) = self.0.read().get(&span) {
                Some(*span)
            } else {
                None
            }
        }
    }
    /// Freeze the set, and return the spans which have been read.
    ///
    /// After this is frozen, no spans that have not been read can be read.
    pub fn freeze_and_get_read_spans(&self) -> UnordMap<Span, Span> {
        self.0
            .freeze()
            .items()
            .filter(|(_, (_, b))| *b)
            .map(|(s1, (s2, _))| (*s1, *s2))
            .collect()
    }
}

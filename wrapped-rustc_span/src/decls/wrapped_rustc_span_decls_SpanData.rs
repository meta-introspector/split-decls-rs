use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents a span.
///
/// Spans represent a region of code, used for error reporting. Positions in spans
/// are *absolute* positions from the beginning of the [`SourceMap`], not positions
/// relative to [`SourceFile`]s. Methods on the `SourceMap` can be used to relate spans back
/// to the original source.
///
/// You must be careful if the span crosses more than one file, since you will not be
/// able to use many of the functions on spans in source_map and you cannot assume
/// that the length of the span is equal to `span.hi - span.lo`; there may be space in the
/// [`BytePos`] range between files.
///
/// `SpanData` is public because `Span` uses a thread-local interner and can't be
/// sent to other threads, but some pieces of performance infra run in a separate thread.
/// Using `Span` is generally preferred.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
#[derive_where(PartialOrd, Ord)]
pub struct SpanData {
    pub lo: BytePos,
    pub hi: BytePos,
    /// Information about where the macro came from, if this piece of
    /// code was created by a macro expansion.
    #[derive_where(skip)]
    pub ctxt: SyntaxContext,
    #[derive_where(skip)]
    pub parent: Option<LocalDefId>,
}

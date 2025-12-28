macro_rules! deps {
    () => {
        Span!();
        SyntaxContext!();
        SourceMap!();
        LocalDefId!();
    };
}

macro_rules! SpanData {
    () => {
        deps!();
        # [doc = " Represents a span."] # [doc = ""] # [doc = " Spans represent a region of code, used for error reporting. Positions in spans"] # [doc = " are *absolute* positions from the beginning of the [`SourceMap`], not positions"] # [doc = " relative to [`SourceFile`]s. Methods on the `SourceMap` can be used to relate spans back"] # [doc = " to the original source."] # [doc = ""] # [doc = " You must be careful if the span crosses more than one file, since you will not be"] # [doc = " able to use many of the functions on spans in source_map and you cannot assume"] # [doc = " that the length of the span is equal to `span.hi - span.lo`; there may be space in the"] # [doc = " [`BytePos`] range between files."] # [doc = ""] # [doc = " `SpanData` is public because `Span` uses a thread-local interner and can't be"] # [doc = " sent to other threads, but some pieces of performance infra run in a separate thread."] # [doc = " Using `Span` is generally preferred."] # [derive (Clone , Copy , Hash , PartialEq , Eq)] # [derive_where (PartialOrd , Ord)] pub struct SpanData { pub lo : BytePos , pub hi : BytePos , # [doc = " Information about where the macro came from, if this piece of"] # [doc = " code was created by a macro expansion."] # [derive_where (skip)] pub ctxt : SyntaxContext , # [derive_where (skip)] pub parent : Option < LocalDefId > , }
    };
}

SpanData!();
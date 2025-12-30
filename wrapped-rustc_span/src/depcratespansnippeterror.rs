// Generated macro for SpanSnippetError (enum)
macro_rules! DepcrateSpanSnippetError {
() => {
// Module: crate
// Provides: {"SpanSnippetError"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Debug)] pub enum SpanSnippetError { IllFormedSpan (Span) , DistinctSources (Box < DistinctSources >) , MalformedForSourcemap (MalformedSourceMapPositions) , SourceNotAvailable { filename : FileName } , }
};
}

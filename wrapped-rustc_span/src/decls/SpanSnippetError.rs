macro_rules! deps {
    () => {
        Span!();
        MalformedSourceMapPositions!();
        FileName!();
        DistinctSources!();
    };
}

macro_rules! SpanSnippetError {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] pub enum SpanSnippetError { IllFormedSpan (Span) , DistinctSources (Box < DistinctSources >) , MalformedForSourcemap (MalformedSourceMapPositions) , SourceNotAvailable { filename : FileName } , }
    };
}

SpanSnippetError!();
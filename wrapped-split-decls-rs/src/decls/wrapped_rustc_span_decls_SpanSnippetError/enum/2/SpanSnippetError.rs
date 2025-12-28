use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Eq , Debug)] pub enum SpanSnippetError { IllFormedSpan (Span) , DistinctSources (Box < DistinctSources >) , MalformedForSourcemap (MalformedSourceMapPositions) , SourceNotAvailable { filename : FileName } , }
macro_rules! deps {
    () => {
        DistinctSources!();
    };
}

macro_rules! SpanLinesError {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] pub enum SpanLinesError { DistinctSources (Box < DistinctSources >) , }
    };
}

SpanLinesError!()
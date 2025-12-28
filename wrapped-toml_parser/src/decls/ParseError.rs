macro_rules! deps {
    () => {
        Expected!();
        ErrorStr!();
        Span!();
    };
}

macro_rules! ParseError {
    () => {
        deps!();
        # [derive (Clone , PartialEq , Eq , Debug)] # [non_exhaustive] pub struct ParseError { context : Option < Span > , description : ErrorStr , expected : Option < & 'static [Expected] > , unexpected : Option < Span > , }
    };
}

ParseError!()
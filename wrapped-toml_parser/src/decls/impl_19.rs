macro_rules! deps {
    () => {
        ErrorStr!();
        ParseError!();
        Span!();
        Expected!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl ParseError { pub fn new (description : impl Into < ErrorStr >) -> Self { Self { context : None , description : description . into () , expected : None , unexpected : None , } } pub fn with_context (mut self , context : Span) -> Self { self . context = Some (context) ; self } pub fn with_expected (mut self , expected : & 'static [Expected]) -> Self { self . expected = Some (expected) ; self } pub fn with_unexpected (mut self , unexpected : Span) -> Self { self . unexpected = Some (unexpected) ; self } pub fn context (& self) -> Option < Span > { self . context } pub fn description (& self) -> & str { & self . description } pub fn expected (& self) -> Option < & 'static [Expected] > { self . expected } pub fn unexpected (& self) -> Option < Span > { self . unexpected } pub (crate) fn rebase_spans (mut self , offset : usize) -> Self { if let Some (context) = self . context . as_mut () { * context += offset ; } if let Some (unexpected) = self . unexpected . as_mut () { * unexpected += offset ; } self } }
    };
}

impl_19!()
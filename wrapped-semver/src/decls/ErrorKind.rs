macro_rules! deps {
    () => {
        Position!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        pub (crate) enum ErrorKind { Empty , UnexpectedEnd (Position) , UnexpectedChar (Position , char) , UnexpectedCharAfter (Position , char) , ExpectedCommaFound (Position , char) , LeadingZero (Position) , Overflow (Position) , EmptySegment (Position) , IllegalCharacter (Position) , WildcardNotTheOnlyComparator (char) , UnexpectedAfterWildcard , ExcessiveComparators , }
    };
}

ErrorKind!()
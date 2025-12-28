macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
        QuotedChar!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { match & self . kind { ErrorKind :: Empty => formatter . write_str ("empty string, expected a semver version") , ErrorKind :: UnexpectedEnd (pos) => { write ! (formatter , "unexpected end of input while parsing {}" , pos) } ErrorKind :: UnexpectedChar (pos , ch) => { write ! (formatter , "unexpected character {} while parsing {}" , QuotedChar (* ch) , pos ,) } ErrorKind :: UnexpectedCharAfter (pos , ch) => { write ! (formatter , "unexpected character {} after {}" , QuotedChar (* ch) , pos ,) } ErrorKind :: ExpectedCommaFound (pos , ch) => { write ! (formatter , "expected comma after {}, found {}" , pos , QuotedChar (* ch) ,) } ErrorKind :: LeadingZero (pos) => { write ! (formatter , "invalid leading zero in {}" , pos) } ErrorKind :: Overflow (pos) => { write ! (formatter , "value of {} exceeds u64::MAX" , pos) } ErrorKind :: EmptySegment (pos) => { write ! (formatter , "empty identifier segment in {}" , pos) } ErrorKind :: IllegalCharacter (pos) => { write ! (formatter , "unexpected character in {}" , pos) } ErrorKind :: WildcardNotTheOnlyComparator (ch) => { write ! (formatter , "wildcard req ({}) must be the only comparator in the version req" , ch ,) } ErrorKind :: UnexpectedAfterWildcard => { formatter . write_str ("unexpected character after wildcard in version req") } ErrorKind :: ExcessiveComparators => { formatter . write_str ("excessive number of version comparators") } } } }
    };
}

impl_15!();
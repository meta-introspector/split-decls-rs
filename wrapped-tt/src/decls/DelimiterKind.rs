macro_rules! DelimiterKind {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum DelimiterKind { Parenthesis , Brace , Bracket , Invisible , }
    };
}

DelimiterKind!()
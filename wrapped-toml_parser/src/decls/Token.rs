macro_rules! deps {
    () => {
        TokenKind!();
        Span!();
    };
}

macro_rules! Token {
    () => {
        deps!();
        # [doc = " An unvalidated TOML Token"] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub struct Token { pub (super) kind : TokenKind , pub (super) span : Span , }
    };
}

Token!();
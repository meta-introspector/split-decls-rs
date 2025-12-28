macro_rules! deps {
    () => {
        ParseBuffer!();
    };
}

macro_rules! Parens {
    () => {
        deps!();
        # [doc (hidden)] pub struct Parens < 'a > { # [doc (hidden)] pub token : token :: Paren , # [doc (hidden)] pub content : ParseBuffer < 'a > , }
    };
}

Parens!();
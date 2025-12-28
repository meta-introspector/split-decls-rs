macro_rules! deps {
    () => {
        ParseBuffer!();
    };
}

macro_rules! Braces {
    () => {
        deps!();
        # [doc (hidden)] pub struct Braces < 'a > { # [doc (hidden)] pub token : token :: Brace , # [doc (hidden)] pub content : ParseBuffer < 'a > , }
    };
}

Braces!();
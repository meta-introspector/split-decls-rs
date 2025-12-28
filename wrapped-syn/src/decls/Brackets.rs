macro_rules! deps {
    () => {
        ParseBuffer!();
    };
}

macro_rules! Brackets {
    () => {
        deps!();
        # [doc (hidden)] pub struct Brackets < 'a > { # [doc (hidden)] pub token : token :: Bracket , # [doc (hidden)] pub content : ParseBuffer < 'a > , }
    };
}

Brackets!()
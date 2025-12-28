macro_rules! deps {
    () => {
        Transparency!();
        SyntaxContext!();
        ExpnId!();
    };
}

macro_rules! SyntaxContextKey {
    () => {
        deps!();
        # [doc = " If this part of two syntax contexts is equal, then the whole syntax contexts should be equal."] # [doc = " The other fields are only for caching."] pub type SyntaxContextKey = (SyntaxContext , ExpnId , Transparency) ;
    };
}

SyntaxContextKey!()
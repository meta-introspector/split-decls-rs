macro_rules! deps {
    () => {
        TokenBuffer!();
        Group!();
        End!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " Internal type which is used instead of `TokenTree` to represent a token tree"] # [doc = " within a `TokenBuffer`."] enum Entry { Group (Group , usize) , Ident (Ident) , Punct (Punct) , Literal (Literal) , End (isize , isize) , }
    };
}

Entry!()
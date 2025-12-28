macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! macro_585 {
    () => {
        deps!();
        ast_struct ! { # [doc = " Arguments of a function path segment: the `(A, B) -> C` in `Fn(A,B) ->"] # [doc = " C`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ParenthesizedGenericArguments { pub paren_token : token :: Paren , # [doc = " `(A, B)`"] pub inputs : Punctuated < Type , Token ! [,] >, # [doc = " `C`"] pub output : ReturnType , } }
    };
}

macro_585!();
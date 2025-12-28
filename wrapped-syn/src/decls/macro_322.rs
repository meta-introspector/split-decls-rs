macro_rules! macro_322 {
    () => {
        ast_struct ! { # [doc = " A trait used as a bound on a type parameter."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct TraitBound { pub paren_token : Option < token :: Paren >, pub modifier : TraitBoundModifier , # [doc = " The `for<'a>` in `for<'a> Foo<&'a T>`"] pub lifetimes : Option < BoundLifetimes >, # [doc = " The `Foo<&'a T>` in `for<'a> Foo<&'a T>`"] pub path : Path , } }
    };
}

macro_322!()
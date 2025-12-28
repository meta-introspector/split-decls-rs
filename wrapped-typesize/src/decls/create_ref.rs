macro_rules! deps {
    () => {
        TypeSize!();
    };
}

macro_rules! create_ref {
    () => {
        deps!();
        macro_rules ! create_ref { ($ (# [$ meta : meta]) ,* pub struct $ name : ident <$ ($ lt : lifetime ,) ? T : ? Sized > (pub $ inner : ty)) => { $ (# [$ meta]) ,* # [doc = concat ! ("A wrapper around `" , stringify ! ($ inner) , "` to implement [`TypeSize`].")] # [doc = ""] # [doc = " This does not consider the size of the inner `T`, simply the size of the pointer."] pub struct $ name <$ ($ lt ,) ? T : ? Sized > (pub $ inner) ; impl <$ ($ lt ,) ? T : ? Sized > TypeSize for $ name <$ ($ lt ,) ? T > { } } ; }
    };
}

create_ref!();
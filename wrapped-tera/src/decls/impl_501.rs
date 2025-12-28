macro_rules! deps {
    () => {
        Context!();
        UserContext!();
    };
}

macro_rules! impl_501 {
    () => {
        deps!();
        impl < 'a > UserContext < 'a > { # [doc = " Create an immutable user context to be used in the call stack"] pub fn new (context : & 'a Context) -> Self { UserContext { inner : context } } pub fn find_value (& self , key : & str) -> Option < & 'a Value > { self . inner . get (key) } pub fn find_value_by_dotted_pointer (& self , pointer : & str) -> Option < & 'a Value > { let root = pointer . split ('.') . next () . unwrap () . replace ("~1" , "/") . replace ("~0" , "~") ; let rest = & pointer [root . len () + 1 ..] ; self . inner . get (& root) . and_then (| val | dotted_pointer (val , rest)) } }
    };
}

impl_501!();
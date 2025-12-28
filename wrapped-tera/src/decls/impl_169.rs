macro_rules! deps {
    () => {
        ExprVal!();
        StringConcat!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl StringConcat { pub (crate) fn to_template_string (& self) -> String { let mut res = Vec :: new () ; for value in & self . values { match value { ExprVal :: String (ref s) => res . push (format ! ("'{}'" , s)) , ExprVal :: Ident (ref s) => res . push (s . to_string ()) , _ => res . push ("unknown" . to_string ()) , } } res . join (" ~ ") } }
    };
}

impl_169!()
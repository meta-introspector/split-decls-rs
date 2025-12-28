macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! In {
    () => {
        deps!();
        # [doc = " Something that checks whether the left side is contained in the right side"] # [derive (Clone , Debug , PartialEq)] pub struct In { # [doc = " The needle, a string or a basic expression/literal"] pub lhs : Box < Expr > , # [doc = " The haystack, can be a string, an array or an ident only currently"] pub rhs : Box < Expr > , # [doc = " Is it using `not` as in `b` not in `...`?"] pub negated : bool , }
    };
}

In!()
macro_rules! deps {
    () => {
        IsLess!();
        IsEqual!();
        IsLessOrEqual!();
        IsNotEqual!();
        IsGreaterOrEqual!();
        IsGreater!();
        B1!();
    };
}

macro_rules! cmp {
    () => {
        deps!();
        # [doc = "\nA convenience macro for comparing type numbers. Use `op!` instead.\n\nDue to the intricacies of the macro system, if the left-hand operand is more complex than a simple\n`ident`, you must place a comma between it and the comparison sign.\n\nFor example, you can do `cmp!(P5 > P3)` or `cmp!(typenum::P5, > typenum::P3)` but not\n`cmp!(typenum::P5 > typenum::P3)`.\n\nThe result of this comparison will always be one of `True` (aka `B1`) or `False` (aka `B0`).\n\n# Example\n```rust\n#[macro_use] extern crate typenum;\nuse typenum::consts::*;\nuse typenum::Bit;\n\nfn main() {\ntype Result = cmp!(P9 == op!(P1 + P2 * (P2 - N2)));\nassert_eq!(Result::to_bool(), true);\n}\n```\n "] # [deprecated (since = "1.9.0" , note = "use the `op!` macro instead")] # [macro_export] macro_rules ! cmp { ($ a : ident < $ b : ty) => { <$ a as $ crate :: IsLess <$ b >>:: Output } ; ($ a : ty , < $ b : ty) => { <$ a as $ crate :: IsLess <$ b >>:: Output } ; ($ a : ident == $ b : ty) => { <$ a as $ crate :: IsEqual <$ b >>:: Output } ; ($ a : ty , == $ b : ty) => { <$ a as $ crate :: IsEqual <$ b >>:: Output } ; ($ a : ident > $ b : ty) => { <$ a as $ crate :: IsGreater <$ b >>:: Output } ; ($ a : ty , > $ b : ty) => { <$ a as $ crate :: IsGreater <$ b >>:: Output } ; ($ a : ident <= $ b : ty) => { <$ a as $ crate :: IsLessOrEqual <$ b >>:: Output } ; ($ a : ty , <= $ b : ty) => { <$ a as $ crate :: IsLessOrEqual <$ b >>:: Output } ; ($ a : ident != $ b : ty) => { <$ a as $ crate :: IsNotEqual <$ b >>:: Output } ; ($ a : ty , != $ b : ty) => { <$ a as $ crate :: IsNotEqual <$ b >>:: Output } ; ($ a : ident >= $ b : ty) => { <$ a as $ crate :: IsGreaterOrEqual <$ b >>:: Output } ; ($ a : ty , >= $ b : ty) => { <$ a as $ crate :: IsGreaterOrEqual <$ b >>:: Output } ; }
    };
}

cmp!();
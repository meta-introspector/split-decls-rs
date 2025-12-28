macro_rules! macro_111 {
    () => {
        declare_lint ! { # [doc = " The `private_interfaces` lint detects types in a primary interface of an item,"] # [doc = " that are more private than the item itself. Primary interface of an item is all"] # [doc = " its interface except for bounds on generic parameters and where clauses."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " #![deny(private_interfaces)]"] # [doc = " struct SemiPriv;"] # [doc = ""] # [doc = " mod m1 {"] # [doc = "     struct Priv;"] # [doc = "     impl crate::SemiPriv {"] # [doc = "         pub fn f(_: Priv) {}"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Having something private in primary interface guarantees that"] # [doc = " the item will be unusable from outer modules due to type privacy."] pub PRIVATE_INTERFACES , Warn , "private type in primary interface of an item" , }
    };
}

macro_111!()
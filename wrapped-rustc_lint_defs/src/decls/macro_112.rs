macro_rules! macro_112 {
    () => {
        declare_lint ! { # [doc = " The `private_bounds` lint detects types in a secondary interface of an item,"] # [doc = " that are more private than the item itself. Secondary interface of an item consists of"] # [doc = " bounds on generic parameters and where clauses, including supertraits for trait items."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " #![deny(private_bounds)]"] # [doc = ""] # [doc = " struct PrivTy;"] # [doc = " pub struct S"] # [doc = "     where PrivTy:"] # [doc = " {}"] # [doc = " # fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Having private types or traits in item bounds makes it less clear what interface"] # [doc = " the item actually provides."] pub PRIVATE_BOUNDS , Warn , "private type in secondary interface of an item" , }
    };
}

macro_112!()
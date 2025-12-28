macro_rules! deps {
    () => {
        Interner!();
        GenericArg!();
        TypeWalker!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < I : Interner > TypeWalker < I > { pub fn new (root : I :: GenericArg) -> Self { Self { stack : smallvec ! [root] , last_subtree : 1 , visited : SsoHashSet :: new () } } # [doc = " Skips the subtree corresponding to the last type"] # [doc = " returned by `next()`."] # [doc = ""] # [doc = " Example: Imagine you are walking `Foo<Bar<i32>, usize>`."] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " let mut iter: TypeWalker = ...;"] # [doc = " iter.next(); // yields Foo"] # [doc = " iter.next(); // yields Bar<i32>"] # [doc = " iter.skip_current_subtree(); // skips i32"] # [doc = " iter.next(); // yields usize"] # [doc = " ```"] pub fn skip_current_subtree (& mut self) { self . stack . truncate (self . last_subtree) ; } }
    };
}

impl_208!()
macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_53 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `unstable_name_collisions` lint detects that you have used a name"] # [doc = " that the standard library plans to add in the future."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " trait MyIterator : Iterator {"] # [doc = "     // is_partitioned is an unstable method that already exists on the Iterator trait"] # [doc = "     fn is_partitioned<P>(self, predicate: P) -> bool"] # [doc = "     where"] # [doc = "         Self: Sized,"] # [doc = "         P: FnMut(Self::Item) -> bool,"] # [doc = "     {true}"] # [doc = " }"] # [doc = ""] # [doc = " impl<T: ?Sized> MyIterator for T where T: Iterator { }"] # [doc = ""] # [doc = " let x = vec![1, 2, 3];"] # [doc = " let _ = x.iter().is_partitioned(|_| true);"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " When new methods are added to traits in the standard library, they are"] # [doc = " usually added in an \"unstable\" form which is only available on the"] # [doc = " [nightly channel] with a [`feature` attribute]. If there is any"] # [doc = " preexisting code which extends a trait to have a method with the same"] # [doc = " name, then the names will collide. In the future, when the method is"] # [doc = " stabilized, this will cause an error due to the ambiguity. This lint"] # [doc = " is an early-warning to let you know that there may be a collision in"] # [doc = " the future. This can be avoided by adding type annotations to"] # [doc = " disambiguate which trait method you intend to call, such as"] # [doc = " `MyIterator::is_partitioned(my_iter, my_predicate)` or renaming or removing the method."] # [doc = ""] # [doc = " [nightly channel]: https://doc.rust-lang.org/book/appendix-07-nightly-rust.html"] # [doc = " [`feature` attribute]: https://doc.rust-lang.org/nightly/unstable-book/"] pub UNSTABLE_NAME_COLLISIONS , Warn , "detects name collision with an existing but unstable method" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: Custom ("once this associated item is added to the standard library, \
             the ambiguity may cause an error or change in behavior!") , reference : "issue #48919 <https://github.com/rust-lang/rust/issues/48919>" , } ; }
    };
}

macro_53!()
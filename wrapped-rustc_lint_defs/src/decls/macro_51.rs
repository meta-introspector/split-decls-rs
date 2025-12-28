macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! macro_51 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `bare_trait_objects` lint suggests using `dyn Trait` for trait"] # [doc = " objects."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2018"] # [doc = " trait Trait { }"] # [doc = ""] # [doc = " fn takes_trait_object(_: Box<Trait>) {"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Without the `dyn` indicator, it can be ambiguous or confusing when"] # [doc = " reading code as to whether or not you are looking at a trait object."] # [doc = " The `dyn` keyword makes it explicit, and adds a symmetry to contrast"] # [doc = " with [`impl Trait`]."] # [doc = ""] # [doc = " [`impl Trait`]: https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters"] pub BARE_TRAIT_OBJECTS , Warn , "suggest using `dyn Trait` for trait objects" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2021) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2021/warnings-promoted-to-error.html>" , } ; }
    };
}

macro_51!()
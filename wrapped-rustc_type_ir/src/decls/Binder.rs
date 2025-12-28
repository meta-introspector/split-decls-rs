macro_rules! deps {
    () => {
        TraitRef!();
        Interner!();
    };
}

macro_rules! Binder {
    () => {
        deps!();
        # [doc = " `Binder` is a binder for higher-ranked lifetimes or types. It is part of the"] # [doc = " compiler's representation for things like `for<'a> Fn(&'a isize)`"] # [doc = " (which would be represented by the type `PolyTraitRef == Binder<I, TraitRef>`)."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/ty_module/instantiating_binders.html>"] # [doc = " for more details."] # [doc = ""] # [doc = " `Decodable` and `Encodable` are implemented for `Binder<T>` using the `impl_binder_encode_decode!` macro."] # [derive_where (Clone , Hash , PartialEq , Debug ; I : Interner , T)] # [derive_where (Copy ; I : Interner , T : Copy)] # [cfg_attr (feature = "nightly" , derive (HashStable_NoContext))] pub struct Binder < I : Interner , T > { value : T , bound_vars : I :: BoundVarKinds , }
    };
}

Binder!()
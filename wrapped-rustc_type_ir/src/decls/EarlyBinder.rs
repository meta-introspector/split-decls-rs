macro_rules! deps {
    () => {
        Interner!();
        Binder!();
    };
}

macro_rules! EarlyBinder {
    () => {
        deps!();
        # [doc = " Similar to [`Binder`] except that it tracks early bound generics, i.e. `struct Foo<T>(T)`"] # [doc = " needs `T` instantiated immediately. This type primarily exists to avoid forgetting to call"] # [doc = " `instantiate`."] # [doc = ""] # [doc = " See <https://rustc-dev-guide.rust-lang.org/ty_module/early_binder.html> for more details."] # [derive_where (Clone , PartialEq , Ord , Hash , Debug ; I : Interner , T)] # [derive_where (PartialOrd ; I : Interner , T : Ord)] # [derive_where (Copy ; I : Interner , T : Copy)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub struct EarlyBinder < I : Interner , T > { value : T , # [derive_where (skip (Debug))] _tcx : PhantomData < fn () -> I > , }
    };
}

EarlyBinder!();
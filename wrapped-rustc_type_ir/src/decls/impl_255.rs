macro_rules! deps {
    () => {
        ArgFolder!();
        TypeFoldable!();
        GenericArg!();
        Interner!();
        EarlyBinder!();
        SliceLike!();
    };
}

macro_rules! impl_255 {
    () => {
        deps!();
        impl < I : Interner , T : TypeFoldable < I > > ty :: EarlyBinder < I , T > { pub fn instantiate < A > (self , cx : I , args : A) -> T where A : SliceLike < Item = I :: GenericArg > , { if args . is_empty () { assert ! (! self . value . has_param () , "{:?} has parameters, but no args were provided in instantiate" , self . value ,) ; return self . value ; } let mut folder = ArgFolder { cx , args : args . as_slice () , binders_passed : 0 } ; self . value . fold_with (& mut folder) } # [doc = " Makes the identity replacement `T0 => T0, ..., TN => TN`."] # [doc = " Conceptually, this converts universally bound variables into placeholders"] # [doc = " when inside of a given item."] # [doc = ""] # [doc = " For example, consider `for<T> fn foo<T>(){ .. }`:"] # [doc = " - Outside of `foo`, `T` is bound (represented by the presence of `EarlyBinder`)."] # [doc = " - Inside of the body of `foo`, we treat `T` as a placeholder by calling"] # [doc = " `instantiate_identity` to discharge the `EarlyBinder`."] pub fn instantiate_identity (self) -> T { self . value } # [doc = " Returns the inner value, but only if it contains no bound vars."] pub fn no_bound_vars (self) -> Option < T > { if ! self . value . has_param () { Some (self . value) } else { None } } }
    };
}

impl_255!()
macro_rules! deps {
    () => {
        Interner!();
        GenericArgs!();
    };
}

macro_rules! ExistentialTraitRef {
    () => {
        deps!();
        # [doc = " An existential reference to a trait, where `Self` is erased."] # [doc = " For example, the trait object `Trait<'a, 'b, X, Y>` is:"] # [doc = " ```ignore (illustrative)"] # [doc = " exists T. T: Trait<'a, 'b, X, Y>"] # [doc = " ```"] # [doc = " The generic parameters don't include the erased `Self`, only trait"] # [doc = " type and lifetime parameters (`[X, Y]` and `['a, 'b]` above)."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ExistentialTraitRef < I : Interner > { pub def_id : I :: TraitId , pub args : I :: GenericArgs , # [doc = " This field exists to prevent the creation of `ExistentialTraitRef` without"] # [doc = " calling [`ExistentialTraitRef::new_from_args`]."] _use_existential_trait_ref_new_instead : () , }
    };
}

ExistentialTraitRef!();
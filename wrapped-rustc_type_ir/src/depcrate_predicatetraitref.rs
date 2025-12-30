// Generated macro for TraitRef (struct)
macro_rules! Depcrate_predicateTraitRef {
() => {
// Module: crate::predicate
// Provides: {"TraitRef"}
// Dependencies: {}
# [doc = " A complete reference to a trait. These take numerous guises in syntax,"] # [doc = " but perhaps the most recognizable form is in a where-clause:"] # [doc = " ```ignore (illustrative)"] # [doc = " T: Foo<U>"] # [doc = " ```"] # [doc = " This would be represented by a trait-reference where the `DefId` is the"] # [doc = " `DefId` for the trait `Foo` and the args define `T` as parameter 0,"] # [doc = " and `U` as parameter 1."] # [doc = ""] # [doc = " Trait references also appear in object types like `Foo<U>`, but in"] # [doc = " that case the `Self` parameter is absent from the generic parameters."] # [derive_where (Clone , Copy , Hash , PartialEq ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct TraitRef < I : Interner > { pub def_id : I :: TraitId , pub args : I :: GenericArgs , # [doc = " This field exists to prevent the creation of `TraitRef` without"] # [doc = " calling [`TraitRef::new_from_args`]."] _use_trait_ref_new_instead : () , }
};
}

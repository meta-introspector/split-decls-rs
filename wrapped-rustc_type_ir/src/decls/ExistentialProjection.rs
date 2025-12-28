macro_rules! deps {
    () => {
        GenericArgs!();
        ExistentialTraitRef!();
        Interner!();
        Term!();
        ProjectionPredicate!();
        DefId!();
    };
}

macro_rules! ExistentialProjection {
    () => {
        deps!();
        # [doc = " A `ProjectionPredicate` for an `ExistentialTraitRef`."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct ExistentialProjection < I : Interner > { pub def_id : I :: DefId , pub args : I :: GenericArgs , pub term : I :: Term , # [doc = " This field exists to prevent the creation of `ExistentialProjection`"] # [doc = " without using [`ExistentialProjection::new_from_args`]."] use_existential_projection_new_instead : () , }
    };
}

ExistentialProjection!();
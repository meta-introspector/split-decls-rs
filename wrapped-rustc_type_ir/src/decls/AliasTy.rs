macro_rules! deps {
    () => {
        Ty!();
        DefId!();
        Interner!();
        GenericArgs!();
        TraitRef!();
    };
}

macro_rules! AliasTy {
    () => {
        deps!();
        # [doc = " Represents the projection of an associated, opaque, or lazy-type-alias type."] # [doc = ""] # [doc = " * For a projection, this would be `<Ty as Trait<...>>::N<...>`."] # [doc = " * For an inherent projection, this would be `Ty::N<...>`."] # [doc = " * For an opaque type, there is no explicit syntax."] # [derive_where (Clone , Copy , Hash , PartialEq , Debug ; I : Interner)] # [derive (TypeVisitable_Generic , TypeFoldable_Generic , Lift_Generic)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub struct AliasTy < I : Interner > { # [doc = " The parameters of the associated or opaque type."] # [doc = ""] # [doc = " For a projection, these are the generic parameters for the trait and the"] # [doc = " GAT parameters, if there are any."] # [doc = ""] # [doc = " For an inherent projection, they consist of the self type and the GAT parameters,"] # [doc = " if there are any."] # [doc = ""] # [doc = " For RPIT the generic parameters are for the generics of the function,"] # [doc = " while for TAIT it is used for the generic parameters of the alias."] pub args : I :: GenericArgs , # [doc = " The `DefId` of the `TraitItem` or `ImplItem` for the associated type `N` depending on whether"] # [doc = " this is a projection or an inherent projection or the `DefId` of the `OpaqueType` item if"] # [doc = " this is an opaque."] # [doc = ""] # [doc = " During codegen, `interner.type_of(def_id)` can be used to get the type of the"] # [doc = " underlying type if the type is an opaque."] # [doc = ""] # [doc = " Note that if this is an associated type, this is not the `DefId` of the"] # [doc = " `TraitRef` containing this associated type, which is in `interner.associated_item(def_id).container`,"] # [doc = " aka. `interner.parent(def_id)`."] pub def_id : I :: DefId , # [doc = " This field exists to prevent the creation of `AliasTy` without using [`AliasTy::new_from_args`]."] # [derive_where (skip (Debug))] pub (crate) _use_alias_ty_new_instead : () , }
    };
}

AliasTy!();
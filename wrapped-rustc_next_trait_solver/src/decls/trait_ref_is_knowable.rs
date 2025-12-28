macro_rules! deps {
    () => {
        OrphanCheckMode!();
        Conflict!();
        InCrate!();
    };
}

macro_rules! trait_ref_is_knowable {
    () => {
        deps!();
        # [doc = " Returns whether all impls which would apply to the `trait_ref`"] # [doc = " e.g. `Ty: Trait<Arg>` are already known in the local crate."] # [doc = ""] # [doc = " This both checks whether any downstream or sibling crates could"] # [doc = " implement it and whether an upstream crate can add this impl"] # [doc = " without breaking backwards compatibility."] # [instrument (level = "debug" , skip (infcx , lazily_normalize_ty) , ret)] pub fn trait_ref_is_knowable < Infcx , I , E > (infcx : & Infcx , trait_ref : ty :: TraitRef < I > , mut lazily_normalize_ty : impl FnMut (I :: Ty) -> Result < I :: Ty , E > ,) -> Result < Result < () , Conflict > , E > where Infcx : InferCtxtLike < Interner = I > , I : Interner , E : Debug , { if orphan_check_trait_ref (infcx , trait_ref , InCrate :: Remote , & mut lazily_normalize_ty) ? . is_ok () { return Ok (Err (Conflict :: Downstream)) ; } if trait_ref_is_local_or_fundamental (infcx . cx () , trait_ref) { return Ok (Ok (())) ; } if orphan_check_trait_ref (infcx , trait_ref , InCrate :: Local { mode : OrphanCheckMode :: Proper } , & mut lazily_normalize_ty ,) ? . is_ok () { Ok (Ok (())) } else { Ok (Err (Conflict :: Upstream)) } }
    };
}

trait_ref_is_knowable!();
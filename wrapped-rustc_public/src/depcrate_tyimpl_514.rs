// Generated macro for impl_514 (impl)
macro_rules! Depcrate_tyimpl_514 {
() => {
// Module: crate::ty
// Provides: {"impl_514"}
// Dependencies: {}
impl AdtDef { pub fn kind (& self) -> AdtKind { with (| cx | cx . adt_kind (* self)) } # [doc = " Retrieve the type of this Adt."] pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } # [doc = " Retrieve the type of this Adt by instantiating and normalizing it with the given arguments."] # [doc = ""] # [doc = " This will assume the type can be instantiated with these arguments."] pub fn ty_with_args (& self , args : & GenericArgs) -> Ty { with (| cx | cx . def_ty_with_args (self . 0 , args)) } pub fn is_box (& self) -> bool { with (| cx | cx . adt_is_box (* self)) } pub fn is_simd (& self) -> bool { with (| cx | cx . adt_is_simd (* self)) } # [doc = " The number of variants in this ADT."] pub fn num_variants (& self) -> usize { with (| cx | cx . adt_variants_len (* self)) } # [doc = " Retrieve the variants in this ADT."] pub fn variants (& self) -> Vec < VariantDef > { self . variants_iter () . collect () } # [doc = " Iterate over the variants in this ADT."] pub fn variants_iter (& self) -> impl Iterator < Item = VariantDef > { (0 .. self . num_variants ()) . map (| idx | VariantDef { idx : VariantIdx :: to_val (idx) , adt_def : * self }) } pub fn variant (& self , idx : VariantIdx) -> Option < VariantDef > { (idx . to_index () < self . num_variants ()) . then_some (VariantDef { idx , adt_def : * self }) } pub fn repr (& self) -> ReprOptions { with (| cx | cx . adt_repr (* self)) } pub fn discriminant_for_variant (& self , idx : VariantIdx) -> Discr { with (| cx | cx . adt_discr_for_variant (* self , idx)) } }
};
}

// Generated macro for record_layout_for_printing (function)
macro_rules! Depcrate_layoutrecord_layout_for_printing {
() => {
// Module: crate::layout
// Provides: {"record_layout_for_printing"}
// Dependencies: {}
fn record_layout_for_printing < 'tcx > (cx : & LayoutCx < 'tcx > , layout : TyAndLayout < 'tcx >) { if layout . ty . has_non_region_param () || ! cx . typing_env . param_env . caller_bounds () . is_empty () { return ; } let record = | kind , packed , opt_discr_size , variants | { let type_desc = with_no_trimmed_paths ! (format ! ("{}" , layout . ty)) ; cx . tcx () . sess . code_stats . record_type_size (kind , type_desc , layout . align . abi , layout . size , packed , opt_discr_size , variants ,) ; } ; match * layout . ty . kind () { ty :: Adt (adt_def , _) => { debug ! ("print-type-size t: `{:?}` process adt" , layout . ty) ; let adt_kind = adt_def . adt_kind () ; let adt_packed = adt_def . repr () . pack . is_some () ; let (variant_infos , opt_discr_size) = variant_info_for_adt (cx , layout , adt_def) ; record (adt_kind . into () , adt_packed , opt_discr_size , variant_infos) ; } ty :: Coroutine (def_id , args) => { debug ! ("print-type-size t: `{:?}` record coroutine" , layout . ty) ; let (variant_infos , opt_discr_size) = variant_info_for_coroutine (cx , layout , def_id , args) ; record (DataTypeKind :: Coroutine , false , opt_discr_size , variant_infos) ; } ty :: Closure (..) => { debug ! ("print-type-size t: `{:?}` record closure" , layout . ty) ; record (DataTypeKind :: Closure , false , None , vec ! []) ; } _ => { debug ! ("print-type-size t: `{:?}` skip non-nominal" , layout . ty) ; } } ; }
};
}

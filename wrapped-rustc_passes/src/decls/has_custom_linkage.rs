macro_rules! has_custom_linkage {
    () => {
        fn has_custom_linkage (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { if ! tcx . def_kind (def_id) . has_codegen_attrs () { return false ; } let codegen_attrs = tcx . codegen_fn_attrs (def_id) ; codegen_attrs . contains_extern_indicator () || codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) || codegen_attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) }
    };
}

has_custom_linkage!();
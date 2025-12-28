macro_rules! deps {
    () => {
        ComesFromAllowExpect!();
    };
}

macro_rules! has_allow_dead_code_or_lang_attr {
    () => {
        deps!();
        fn has_allow_dead_code_or_lang_attr (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < ComesFromAllowExpect > { fn has_lang_attr (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . has_attr (def_id , sym :: lang) || tcx . has_attr (def_id , sym :: panic_handler) } fn has_allow_expect_dead_code (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let lint_level = tcx . lint_level_at_node (lint :: builtin :: DEAD_CODE , hir_id) . level ; matches ! (lint_level , lint :: Allow | lint :: Expect) } fn has_used_like_attr (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . def_kind (def_id) . has_codegen_attrs () && { let cg_attrs = tcx . codegen_fn_attrs (def_id) ; cg_attrs . contains_extern_indicator () || cg_attrs . flags . contains (CodegenFnAttrFlags :: USED_COMPILER) || cg_attrs . flags . contains (CodegenFnAttrFlags :: USED_LINKER) } } if has_allow_expect_dead_code (tcx , def_id) { Some (ComesFromAllowExpect :: Yes) } else if has_used_like_attr (tcx , def_id) || has_lang_attr (tcx , def_id) { Some (ComesFromAllowExpect :: No) } else { None } }
    };
}

has_allow_dead_code_or_lang_attr!()
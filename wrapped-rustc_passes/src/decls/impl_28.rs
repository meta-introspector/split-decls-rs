macro_rules! deps {
    () => {
        UnexportableItem!();
        ExportableItemsChecker!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < 'tcx , 'a > ExportableItemsChecker < 'tcx , 'a > { fn check (& mut self) { match self . tcx . def_kind (self . item_id) { DefKind :: Fn | DefKind :: AssocFn => self . check_fn () , DefKind :: Enum | DefKind :: Struct | DefKind :: Union => self . check_ty () , _ => { } } } fn check_fn (& mut self) { let def_id = self . item_id . expect_local () ; let span = self . tcx . def_span (def_id) ; if self . tcx . generics_of (def_id) . requires_monomorphization (self . tcx) { self . tcx . dcx () . emit_err (UnexportableItem :: GenericFn (span)) ; return ; } let sig = self . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () ; if ! matches ! (sig . abi , ExternAbi :: C { .. }) { self . tcx . dcx () . emit_err (UnexportableItem :: FnAbi (span)) ; return ; } let sig = self . tcx . try_normalize_erasing_regions (ty :: TypingEnv :: non_body_analysis (self . tcx , def_id) , sig) . unwrap_or (sig) ; let hir_id = self . tcx . local_def_id_to_hir_id (def_id) ; let decl = self . tcx . hir_fn_decl_by_hir_id (hir_id) . unwrap () ; for (input_ty , input_hir) in iter :: zip (sig . inputs () , decl . inputs) { self . check_nested_types_are_exportable (* input_ty , input_hir . span) ; } if let hir :: FnRetTy :: Return (ret_hir) = decl . output { self . check_nested_types_are_exportable (sig . output () , ret_hir . span) ; } } fn check_ty (& mut self) { let ty = self . tcx . type_of (self . item_id) . skip_binder () ; if let ty :: Adt (adt_def , _) = ty . kind () { if ! adt_def . repr () . inhibit_struct_field_reordering () { self . tcx . dcx () . emit_err (UnexportableItem :: TypeRepr (self . tcx . def_span (self . item_id))) ; } for variant in adt_def . variants () { for field in & variant . fields { if ! field . vis . is_public () { self . tcx . dcx () . emit_err (UnexportableItem :: AdtWithPrivFields { span : self . tcx . def_span (self . item_id) , vis_note : self . tcx . def_span (field . did) , field_name : field . name . as_str () , }) ; } } } } } fn check_nested_types_are_exportable (& mut self , ty : Ty < 'tcx > , ty_span : Span) { let res = ty . visit_with (self) ; if let Some (err_cause) = res . break_value () { self . tcx . dcx () . emit_err (UnexportableItem :: TypeInInterface { span : self . tcx . def_span (self . item_id) , desc : self . tcx . def_descr (self . item_id) , ty : & format ! ("{}" , err_cause) , ty_span , }) ; } } }
    };
}

impl_28!();
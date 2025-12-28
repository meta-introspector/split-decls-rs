macro_rules! deps {
    () => {
        RustcConstStableIndirectPairing!();
        MissingStabilityAnnotations!();
        AnnotationKind!();
        CannotStabilizeDeprecated!();
        UselessStability!();
        MissingConstErr!();
        MissingConstStabAttr!();
        MissingStabilityAttr!();
        DeprecatedAttribute!();
        ConstStableNotStable!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < 'tcx > MissingStabilityAnnotations < 'tcx > { # [doc = " Verify that deprecation and stability attributes make sense with one another."] # [instrument (level = "trace" , skip (self))] fn check_compatible_stability (& self , def_id : LocalDefId) { if ! self . tcx . features () . staged_api () { return ; } let depr = self . tcx . lookup_deprecation_entry (def_id) ; let stab = self . tcx . lookup_stability (def_id) ; let const_stab = self . tcx . lookup_const_stability (def_id) ; macro_rules ! find_attr_span { ($ name : ident) => { { let attrs = self . tcx . hir_attrs (self . tcx . local_def_id_to_hir_id (def_id)) ; find_attr ! (attrs , AttributeKind ::$ name { span , .. } => * span) } } } if stab . is_none () && depr . map_or (false , | d | d . attr . is_since_rustc_version ()) && let Some (span) = find_attr_span ! (Deprecation) { self . tcx . dcx () . emit_err (errors :: DeprecatedAttribute { span }) ; } if let Some (stab) = stab { let kind = annotation_kind (self . tcx , def_id) ; if kind == AnnotationKind :: Prohibited || (kind == AnnotationKind :: Container && stab . level . is_stable () && depr . is_some ()) { if let Some (span) = find_attr_span ! (Stability) { let item_sp = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (errors :: UselessStability { span , item_sp }) ; } } if let Some (depr) = depr && let DeprecatedSince :: RustcVersion (dep_since) = depr . attr . since && let StabilityLevel :: Stable { since : stab_since , .. } = stab . level && let Some (span) = find_attr_span ! (Stability) { let item_sp = self . tcx . def_span (def_id) ; match stab_since { StableSince :: Current => { self . tcx . dcx () . emit_err (errors :: CannotStabilizeDeprecated { span , item_sp }) ; } StableSince :: Version (stab_since) => { if dep_since < stab_since { self . tcx . dcx () . emit_err (errors :: CannotStabilizeDeprecated { span , item_sp }) ; } } StableSince :: Err (_) => { } } } } let fn_sig = self . tcx . hir_node_by_def_id (def_id) . fn_sig () ; if let Some (fn_sig) = fn_sig && ! fn_sig . header . is_const () && const_stab . is_some () && find_attr_span ! (ConstStability) . is_some () { self . tcx . dcx () . emit_err (errors :: MissingConstErr { fn_sig_span : fn_sig . span }) ; } if let Some (const_stab) = const_stab && let Some (fn_sig) = fn_sig && const_stab . is_const_stable () && ! stab . is_some_and (| s | s . is_stable ()) && let Some (const_span) = find_attr_span ! (ConstStability) { self . tcx . dcx () . emit_err (errors :: ConstStableNotStable { fn_sig_span : fn_sig . span , const_span }) ; } if let Some (stab) = & const_stab && stab . is_const_stable () && stab . const_stable_indirect && let Some (span) = find_attr_span ! (ConstStability) { self . tcx . dcx () . emit_err (errors :: RustcConstStableIndirectPairing { span }) ; } } # [instrument (level = "debug" , skip (self))] fn check_missing_stability (& self , def_id : LocalDefId) { let stab = self . tcx . lookup_stability (def_id) ; self . tcx . ensure_ok () . lookup_const_stability (def_id) ; if ! self . tcx . sess . is_test_crate () && stab . is_none () && self . effective_visibilities . is_reachable (def_id) { let descr = self . tcx . def_descr (def_id . to_def_id ()) ; let span = self . tcx . def_span (def_id) ; self . tcx . dcx () . emit_err (errors :: MissingStabilityAttr { span , descr }) ; } } fn check_missing_const_stability (& self , def_id : LocalDefId) { let is_const = self . tcx . is_const_fn (def_id . to_def_id ()) || (self . tcx . def_kind (def_id . to_def_id ()) == DefKind :: Trait && self . tcx . is_const_trait (def_id . to_def_id ())) ; if is_const && self . effective_visibilities . is_reachable (def_id) && self . tcx . lookup_const_stability (def_id) . is_none () { let span = self . tcx . def_span (def_id) ; let descr = self . tcx . def_descr (def_id . to_def_id ()) ; self . tcx . dcx () . emit_err (errors :: MissingConstStabAttr { span , descr }) ; } } }
    };
}

impl_328!()
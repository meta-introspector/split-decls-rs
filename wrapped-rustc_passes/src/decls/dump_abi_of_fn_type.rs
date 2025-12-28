macro_rules! deps {
    () => {
        UnrecognizedArgument!();
        AbiOf!();
        AbiNe!();
    };
}

macro_rules! dump_abi_of_fn_type {
    () => {
        deps!();
        fn dump_abi_of_fn_type (tcx : TyCtxt < '_ > , item_def_id : LocalDefId , attr : & Attribute) { let typing_env = ty :: TypingEnv :: post_analysis (tcx , item_def_id) ; let ty = tcx . type_of (item_def_id) . instantiate_identity () ; let span = tcx . def_span (item_def_id) ; if ! ensure_wf (tcx , typing_env , ty , item_def_id , span) { return ; } let meta_items = attr . meta_item_list () . unwrap_or_default () ; for meta_item in meta_items { match meta_item . name () { Some (sym :: debug) => { let ty :: FnPtr (sig_tys , hdr) = ty . kind () else { span_bug ! (meta_item . span () , "`#[rustc_abi(debug)]` on a type alias requires function pointer type") ; } ; let abi = unwrap_fn_abi (tcx . fn_abi_of_fn_ptr (typing_env . as_query_input ((sig_tys . with (* hdr) , ty :: List :: empty () ,))) , tcx , item_def_id ,) ; let fn_name = tcx . item_name (item_def_id) ; tcx . dcx () . emit_err (AbiOf { span , fn_name , fn_abi : format ! ("{:#?}" , abi) }) ; } Some (sym :: assert_eq) => { let ty :: Tuple (fields) = ty . kind () else { span_bug ! (meta_item . span () , "`#[rustc_abi(assert_eq)]` on a type alias requires pair type") ; } ; let [field1 , field2] = * * * fields else { span_bug ! (meta_item . span () , "`#[rustc_abi(assert_eq)]` on a type alias requires pair type") ; } ; let ty :: FnPtr (sig_tys1 , hdr1) = field1 . kind () else { span_bug ! (meta_item . span () , "`#[rustc_abi(assert_eq)]` on a type alias requires pair of function pointer types") ; } ; let abi1 = unwrap_fn_abi (tcx . fn_abi_of_fn_ptr (typing_env . as_query_input ((sig_tys1 . with (* hdr1) , ty :: List :: empty () ,))) , tcx , item_def_id ,) ; let ty :: FnPtr (sig_tys2 , hdr2) = field2 . kind () else { span_bug ! (meta_item . span () , "`#[rustc_abi(assert_eq)]` on a type alias requires pair of function pointer types") ; } ; let abi2 = unwrap_fn_abi (tcx . fn_abi_of_fn_ptr (typing_env . as_query_input ((sig_tys2 . with (* hdr2) , ty :: List :: empty () ,))) , tcx , item_def_id ,) ; if ! test_abi_eq (abi1 , abi2) { tcx . dcx () . emit_err (AbiNe { span , left : format ! ("{:#?}" , abi1) , right : format ! ("{:#?}" , abi2) , }) ; } } _ => { tcx . dcx () . emit_err (UnrecognizedArgument { span : meta_item . span () }) ; } } } }
    };
}

dump_abi_of_fn_type!()
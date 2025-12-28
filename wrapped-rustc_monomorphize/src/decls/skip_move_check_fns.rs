macro_rules! skip_move_check_fns {
    () => {
        pub (crate) fn skip_move_check_fns (tcx : TyCtxt < '_ > , _ : ()) -> FxIndexSet < DefId > { let fns = [(tcx . lang_items () . owned_box () , "new") , (tcx . get_diagnostic_item (sym :: Rc) , "new") , (tcx . get_diagnostic_item (sym :: Arc) , "new") ,] ; fns . into_iter () . filter_map (| (def_id , fn_name) | { def_id . and_then (| def_id | assoc_fn_of_type (tcx , def_id , Ident :: from_str (fn_name))) }) . collect () }
    };
}

skip_move_check_fns!()
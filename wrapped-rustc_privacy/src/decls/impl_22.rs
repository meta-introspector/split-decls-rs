macro_rules! deps {
    () => {
        VisibilityLike!();
        FindMin!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl VisibilityLike for EffectiveVisibility { const MAX : Self = EffectiveVisibility :: from_vis (ty :: Visibility :: Public) ; fn new_min < const SHALLOW : bool > (find : & FindMin < '_ , '_ , Self , SHALLOW > , def_id : LocalDefId ,) -> Self { let effective_vis = find . effective_visibilities . effective_vis (def_id) . copied () . unwrap_or_else (| | { let private_vis = ty :: Visibility :: Restricted (find . tcx . parent_module_from_def_id (def_id) . to_local_def_id () ,) ; EffectiveVisibility :: from_vis (private_vis) }) ; effective_vis . min (find . min , find . tcx) } }
    };
}

impl_22!()
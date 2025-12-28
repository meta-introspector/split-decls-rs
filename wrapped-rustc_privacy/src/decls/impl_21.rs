macro_rules! deps {
    () => {
        VisibilityLike!();
        FindMin!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl VisibilityLike for ty :: Visibility { const MAX : Self = ty :: Visibility :: Public ; fn new_min < const SHALLOW : bool > (find : & FindMin < '_ , '_ , Self , SHALLOW > , def_id : LocalDefId ,) -> Self { min (find . tcx . local_visibility (def_id) , find . min , find . tcx) } }
    };
}

impl_21!();
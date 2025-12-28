macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: interpret :: AllocId { type T = crate :: mir :: alloc :: AllocId ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , _ : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . create_alloc_id (* self) } }
    };
}

impl_157!()
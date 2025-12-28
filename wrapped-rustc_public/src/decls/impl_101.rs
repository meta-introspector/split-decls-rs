macro_rules! deps {
    () => {
        Layout!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Layout < 'tcx > { type T = Layout ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . layout_id (cx . lift (* self) . unwrap ()) } }
    };
}

impl_101!()
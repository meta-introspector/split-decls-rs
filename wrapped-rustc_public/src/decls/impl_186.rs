macro_rules! deps {
    () => {
        BridgeTys!();
        Ty!();
        Stable!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for Ty < 'tcx > { type T = crate :: ty :: Ty ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { tables . intern_ty (cx . lift (* self) . unwrap ()) } }
    };
}

impl_186!()
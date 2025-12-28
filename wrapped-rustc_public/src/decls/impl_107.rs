macro_rules! deps {
    () => {
        Stable!();
        FieldsShape!();
        Primitive!();
        BridgeTys!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: FieldsShape < rustc_abi :: FieldIdx > { type T = FieldsShape ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: FieldsShape :: Primitive => FieldsShape :: Primitive , rustc_abi :: FieldsShape :: Union (count) => FieldsShape :: Union (* count) , rustc_abi :: FieldsShape :: Array { stride , count } => { FieldsShape :: Array { stride : stride . stable (tables , cx) , count : * count } } rustc_abi :: FieldsShape :: Arbitrary { offsets , .. } => { FieldsShape :: Arbitrary { offsets : offsets . iter () . as_slice () . stable (tables , cx) } } } } }
    };
}

impl_107!()
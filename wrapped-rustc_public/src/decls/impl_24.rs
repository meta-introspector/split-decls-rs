macro_rules! deps {
    () => {
        FieldsShape!();
        Primitive!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl FieldsShape { pub fn fields_by_offset_order (& self) -> Vec < FieldIdx > { match self { FieldsShape :: Primitive => vec ! [] , FieldsShape :: Union (_) | FieldsShape :: Array { .. } => (0 .. self . count ()) . collect () , FieldsShape :: Arbitrary { offsets , .. } => { let mut indices = (0 .. offsets . len ()) . collect :: < Vec < _ > > () ; indices . sort_by_key (| idx | offsets [* idx]) ; indices } } } pub fn count (& self) -> usize { match self { FieldsShape :: Primitive => 0 , FieldsShape :: Union (count) => count . get () , FieldsShape :: Array { count , .. } => * count as usize , FieldsShape :: Arbitrary { offsets , .. } => offsets . len () , } } }
    };
}

impl_24!();
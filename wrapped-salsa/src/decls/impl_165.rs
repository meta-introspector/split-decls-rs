macro_rules! deps {
    () => {
        FromId!();
        Value!();
        StructEntry!();
        Configuration!();
        DatabaseKeyIndex!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'db , C > StructEntry < 'db , C > where C : Configuration , { # [doc = " Returns the `DatabaseKeyIndex` for this entry."] pub fn key (& self) -> DatabaseKeyIndex { self . key } # [doc = " Returns the input struct."] pub fn as_struct (& self) -> C :: Struct { FromId :: from_id (self . key . key_index ()) } # [cfg (feature = "salsa_unstable")] pub fn value (& self) -> & 'db Value < C > { self . value } }
    };
}

impl_165!();
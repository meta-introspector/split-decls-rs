macro_rules! deps {
    () => {
        Configuration!();
        DatabaseKeyIndex!();
        StructEntry!();
        FromId!();
        Value!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'db , C > StructEntry < 'db , C > where C : Configuration , { # [doc = " Returns the `DatabaseKeyIndex` for this entry."] pub fn key (& self) -> DatabaseKeyIndex { self . key } # [doc = " Returns the interned struct."] pub fn as_struct (& self) -> C :: Struct < '_ > { FromId :: from_id (self . key . key_index ()) } # [cfg (feature = "salsa_unstable")] pub fn value (& self) -> & 'db Value < C > { self . value } }
    };
}

impl_191!();
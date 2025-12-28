macro_rules! deps {
    () => {
        SerializeFieldMap!();
    };
}

macro_rules! AsMap {
    () => {
        deps!();
        pub trait AsMap : Sized + sealed :: Sealed { fn field_map (& self) -> SerializeFieldMap < '_ , Self > { SerializeFieldMap (self) } }
    };
}

AsMap!();
macro_rules! deps {
    () => {
        Serializer!();
        SerdeVariable!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Serialize for SerdeVariable { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let mut s = serializer . serialize_map (Some (1)) ? ; s . serialize_entry ("$var" , & self . 0) ? ; s . end () } }
    };
}

impl_70!()
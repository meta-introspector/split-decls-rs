macro_rules! deps {
    () => {
        Serializer!();
        Name!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Serialize for Name { fn serialize < S : Serializer > (& self , serializer : S) -> std :: result :: Result < S :: Ok , S :: Error > { serializer . serialize_str (& self . 0) } }
    };
}

impl_84!();
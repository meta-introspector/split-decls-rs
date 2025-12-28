macro_rules! deps {
    () => {
        TextRange!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl Serialize for TextRange { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { (self . start () , self . end ()) . serialize (serializer) } }
    };
}

impl_45!();
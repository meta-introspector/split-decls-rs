macro_rules! deps {
    () => {
        Time!();
        Datetime!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "serde")] # [cfg (feature = "alloc")] impl serde_core :: ser :: Serialize for Time { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { Datetime :: from (* self) . serialize (serializer) } }
    };
}

impl_32!()
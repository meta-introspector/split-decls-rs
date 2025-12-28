macro_rules! deps {
    () => {
        Result!();
        Unique!();
        UniqueStrategy!();
        GetValue!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < K : GetValue + Eq + std :: hash :: Hash > UniqueStrategy for Unique < K > { fn insert (& mut self , val : & Value) -> Result < bool > { Ok (self . unique . insert (K :: get_value (val) ?)) } }
    };
}

impl_153!()
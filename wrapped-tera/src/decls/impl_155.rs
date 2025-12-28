macro_rules! deps {
    () => {
        UniqueStrategy!();
        Result!();
        UniqueStrings!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl UniqueStrategy for UniqueStrings { fn insert (& mut self , val : & Value) -> Result < bool > { let mut key = String :: get_value (val) ? ; if ! self . case_sensitive { key = key . to_lowercase () } Ok (self . u . unique . insert (key)) } }
    };
}

impl_155!()
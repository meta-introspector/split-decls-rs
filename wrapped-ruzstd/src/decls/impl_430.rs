macro_rules! deps {
    () => {
        Read!();
        Take!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl < R : Read > Take < R > { pub fn limit (& self) -> u64 { self . limit } pub fn set_limit (& mut self , limit : u64) { self . limit = limit ; } pub fn get_ref (& self) -> & R { & self . inner } pub fn get_mut (& mut self) -> & mut R { & mut self . inner } pub fn into_inner (self) -> R { self . inner } }
    };
}

impl_430!()
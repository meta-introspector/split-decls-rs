macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Data { pub fn new (attrs : & Attributes < '_ > , written : bool) -> Self { let mut span = Self { start : Instant :: now () , kvs : Vec :: new () , written , } ; attrs . record (& mut span) ; span } }
    };
}

impl_3!()
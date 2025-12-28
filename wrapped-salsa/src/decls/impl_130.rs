macro_rules! deps {
    () => {
        TypeIdHasher!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl Hasher for TypeIdHasher { fn write (& mut self , _ : & [u8]) { unreachable ! ("`TypeId` calls `write_u64`") ; } # [inline] fn write_u64 (& mut self , id : u64) { self . 0 = id ; } # [inline] fn finish (& self) -> u64 { self . 0 } }
    };
}

impl_130!()
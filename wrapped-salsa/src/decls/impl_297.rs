macro_rules! deps {
    () => {
        Storage!();
        Database!();
        ZalsaLocal!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl < Db : Database > Clone for Storage < Db > { fn clone (& self) -> Self { Self { handle : self . handle . clone () , zalsa_local : ZalsaLocal :: new () , } } }
    };
}

impl_297!();
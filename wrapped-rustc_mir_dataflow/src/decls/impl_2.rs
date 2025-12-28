macro_rules! deps {
    () => {
        DebuginfoLocals!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Visitor < '_ > for DebuginfoLocals { fn visit_local (& mut self , local : Local , _ : PlaceContext , _ : Location) { self . 0 . insert (local) ; } }
    };
}

impl_2!()
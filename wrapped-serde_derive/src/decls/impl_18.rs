macro_rules! deps {
    () => {
        Attr!();
        BoolAttr!();
        Ctxt!();
        Symbol!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'c > BoolAttr < 'c > { fn none (cx : & 'c Ctxt , name : Symbol) -> Self { BoolAttr (Attr :: none (cx , name)) } fn set_true < A : ToTokens > (& mut self , obj : A) { self . 0 . set (obj , ()) ; } fn get (& self) -> bool { self . 0 . value . is_some () } }
    };
}

impl_18!()
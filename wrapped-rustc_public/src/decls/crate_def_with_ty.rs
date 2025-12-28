macro_rules! deps {
    () => {
        CrateDefType!();
        DefId!();
        CrateDef!();
    };
}

macro_rules! crate_def_with_ty {
    () => {
        deps!();
        macro_rules ! crate_def_with_ty { ($ (# [$ attr : meta]) * $ vis : vis $ name : ident $ (;) ?) => { $ (# [$ attr]) * # [derive (Clone , Copy , PartialEq , Eq , Debug , Hash)] $ vis struct $ name (pub DefId) ; impl CrateDef for $ name { fn def_id (& self) -> DefId { self . 0 } } impl CrateDefType for $ name { } } ; }
    };
}

crate_def_with_ty!()
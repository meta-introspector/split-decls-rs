macro_rules! TyQ {
    () => {
        # [doc = " Type and extended type qualifiers."] # [derive (Eq , Hash , PartialEq)] pub (crate) enum TyQ { None , Const , Mut , }
    };
}

TyQ!()
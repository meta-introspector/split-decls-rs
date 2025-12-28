macro_rules! macro_42 {
    () => {
        bridge_impl ! (FnDef , crate :: ty :: FnDef) ;
    };
}

macro_42!()
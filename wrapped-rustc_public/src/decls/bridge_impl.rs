macro_rules! deps {
    () => {
        BridgeTys!();
        DefId!();
    };
}

macro_rules! bridge_impl {
    () => {
        deps!();
        macro_rules ! bridge_impl { ($ name : ident , $ ty : ty) => { impl rustc_public_bridge :: bridge ::$ name < compiler_interface :: BridgeTys > for $ ty { fn new (def : crate :: DefId) -> Self { Self (def) } } } ; }
    };
}

bridge_impl!();
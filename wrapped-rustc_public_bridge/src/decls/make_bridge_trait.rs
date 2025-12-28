macro_rules! deps {
    () => {
        Bridge!();
    };
}

macro_rules! make_bridge_trait {
    () => {
        deps!();
        macro_rules ! make_bridge_trait { ($ name : ident) => { pub trait $ name < B : Bridge > { fn new (did : B :: DefId) -> Self ; } } ; }
    };
}

make_bridge_trait!()
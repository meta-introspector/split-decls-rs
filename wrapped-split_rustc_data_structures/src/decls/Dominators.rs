macro_rules! deps {
    () => {
        Kind!();
        Node!();
    };
}

macro_rules! Dominators {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct Dominators < Node : Idx > { kind : Kind < Node > , }
    };
}

Dominators!()
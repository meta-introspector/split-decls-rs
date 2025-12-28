macro_rules! deps {
    () => {
        Node!();
        Kind!();
    };
}

macro_rules! Dominators {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct Dominators < Node : Idx > { kind : Kind < Node > , }
    };
}

Dominators!();
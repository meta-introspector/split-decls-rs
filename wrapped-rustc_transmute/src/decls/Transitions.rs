macro_rules! deps {
    () => {
        State!();
        Type!();
        Region!();
        Reference!();
    };
}

macro_rules! Transitions {
    () => {
        deps!();
        # [derive (PartialEq , Clone , Debug)] pub (crate) struct Transitions < R , T > where R : Region , T : Type , { byte_transitions : EdgeSet < State > , ref_transitions : Map < Reference < R , T > , State > , }
    };
}

Transitions!();
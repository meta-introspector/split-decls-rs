macro_rules! deps {
    () => {
        Interner!();
        ProbeStep!();
        ProbeKind!();
        CanonicalState!();
    };
}

macro_rules! Probe {
    () => {
        deps!();
        # [doc = " A self-contained computation during trait solving. This either"] # [doc = " corresponds to a `EvalCtxt::probe(_X)` call or the root evaluation"] # [doc = " of a goal."] # [derive_where (PartialEq , Eq , Hash , Debug ; I : Interner)] pub struct Probe < I : Interner > { # [doc = " What happened inside of this probe in chronological order."] pub steps : Vec < ProbeStep < I > > , pub kind : ProbeKind < I > , pub final_state : CanonicalState < I , () > , }
    };
}

Probe!();
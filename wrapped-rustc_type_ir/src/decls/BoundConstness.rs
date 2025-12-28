macro_rules! deps {
    () => {
        Const!();
    };
}

macro_rules! BoundConstness {
    () => {
        deps!();
        # [derive (Clone , Copy , Hash , PartialEq , Eq , Debug)] # [cfg_attr (feature = "nightly" , derive (Encodable_NoContext , Decodable_NoContext , HashStable_NoContext))] pub enum BoundConstness { # [doc = " `Type: const Trait`"] # [doc = ""] # [doc = " A bound is required to be unconditionally const, even in a runtime function."] Const , # [doc = " `Type: [const] Trait`"] # [doc = ""] # [doc = " Requires resolving to const only when we are in a const context."] Maybe , }
    };
}

BoundConstness!();
macro_rules! deps {
    () => {
        BoundVariableKind!();
    };
}

macro_rules! Binder {
    () => {
        deps!();
        # [doc = " A binder represents a possibly generic type and its bound vars."] # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct Binder < T > { pub value : T , pub bound_vars : Vec < BoundVariableKind > , }
    };
}

Binder!();
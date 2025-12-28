macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! Workspace {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub (crate) struct Workspace { # [serde (skip_serializing_if = "Map::is_empty")] pub dependencies : Map < String , Dependency > , }
    };
}

Workspace!();
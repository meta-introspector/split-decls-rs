macro_rules! deps {
    () => {
        Dependency!();
        WorkspacePackage!();
    };
}

macro_rules! WorkspaceWorkspace {
    () => {
        deps!();
        # [derive (Deserialize , Default , Debug)] pub (crate) struct WorkspaceWorkspace { # [serde (default)] pub package : WorkspacePackage , # [serde (default)] pub dependencies : Map < String , Dependency > , }
    };
}

WorkspaceWorkspace!();
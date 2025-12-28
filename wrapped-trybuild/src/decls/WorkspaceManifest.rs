macro_rules! deps {
    () => {
        WorkspaceWorkspace!();
        RegistryPatch!();
        Patch!();
    };
}

macro_rules! WorkspaceManifest {
    () => {
        deps!();
        # [derive (Deserialize , Default , Debug)] pub (crate) struct WorkspaceManifest { # [serde (default)] pub workspace : WorkspaceWorkspace , # [serde (default)] pub patch : Map < String , RegistryPatch > , # [serde (default)] pub replace : Map < String , Patch > , }
    };
}

WorkspaceManifest!();
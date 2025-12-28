macro_rules! deps {
    () => {
        Dependency!();
        Bin!();
        Patch!();
        Workspace!();
        Package!();
        RegistryPatch!();
        TargetDependencies!();
    };
}

macro_rules! Manifest {
    () => {
        deps!();
        # [derive (Serialize , Debug)] pub (crate) struct Manifest { # [serde (rename = "cargo-features" , skip_serializing_if = "Vec::is_empty")] pub cargo_features : Vec < String > , pub package : Package , # [serde (skip_serializing_if = "Map::is_empty")] pub features : Map < String , Vec < String > > , pub dependencies : Map < String , Dependency > , # [serde (skip_serializing_if = "Map::is_empty")] pub target : Map < String , TargetDependencies > , # [serde (rename = "bin")] pub bins : Vec < Bin > , # [serde (skip_serializing_if = "Option::is_none")] pub workspace : Option < Workspace > , # [serde (serialize_with = "serialize_patch" , skip_serializing_if = "empty_patch")] pub patch : Map < String , RegistryPatch > , # [serde (skip_serializing_if = "Map::is_empty")] pub replace : Map < String , Patch > , }
    };
}

Manifest!()
macro_rules! deps {
    () => {
        BuildTarget!();
    };
}

macro_rules! PackageMetadata {
    () => {
        deps!();
        # [derive (Deserialize)] pub (crate) struct PackageMetadata { pub name : String , pub targets : Vec < BuildTarget > , pub manifest_path : PathBuf , }
    };
}

PackageMetadata!();
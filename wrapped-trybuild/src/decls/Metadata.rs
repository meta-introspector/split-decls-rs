macro_rules! deps {
    () => {
        PackageMetadata!();
        Directory!();
    };
}

macro_rules! Metadata {
    () => {
        deps!();
        # [derive (Deserialize)] pub (crate) struct Metadata { pub target_directory : Directory , pub workspace_root : Directory , pub packages : Vec < PackageMetadata > , }
    };
}

Metadata!();
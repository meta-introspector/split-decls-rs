macro_rules! deps {
    () => {
        ExpandedFileEntry!();
    };
}

macro_rules! ExpandedManifest {
    () => {
        deps!();
        # [derive (Debug , Serialize , Deserialize)] pub struct ExpandedManifest { pub rustc_version : String , pub rustc_host : String , pub project_root : PathBuf , pub expanded_files : Vec < ExpandedFileEntry > , }
    };
}

ExpandedManifest!();
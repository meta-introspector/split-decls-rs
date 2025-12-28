macro_rules! ExpandedFileEntry {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub struct ExpandedFileEntry { pub package_name : String , pub target_type : String , pub target_name : String , pub expanded_rs_path : PathBuf , pub cargo_expand_command : String , pub timestamp : u64 , pub flake_lock_details : serde_json :: Value , pub layer : u32 , }
    };
}

ExpandedFileEntry!()
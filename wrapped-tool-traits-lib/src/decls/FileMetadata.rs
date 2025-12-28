macro_rules! FileMetadata {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct FileMetadata { pub modified : SystemTime , pub len : u64 , pub hash : String , pub git_object_hash : Option < String > , pub is_git_tracked : bool , }
    };
}

FileMetadata!();
macro_rules! FileMetadata {
    () => {
        # [derive (Debug , Default)] pub struct FileMetadata { pub global_uses : HashSet < String > , pub feature_attributes : HashSet < String > , pub extern_crates : HashSet < String > , }
    };
}

FileMetadata!()
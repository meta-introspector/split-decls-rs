macro_rules! PublicSymbol {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct PublicSymbol { pub identifier : String , pub declaration_type : String , pub signature : String , pub source_file : PathBuf , pub crate_name : String , }
    };
}

PublicSymbol!();
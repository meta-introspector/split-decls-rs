macro_rules! SerializableDeclaration {
    () => {
        # [derive (Debug , Serialize , Deserialize)] pub struct SerializableDeclaration { pub identifier : String , pub declaration_type : String , pub referenced_types : HashSet < String > , pub referenced_functions : HashSet < String > , pub external_identifiers : HashSet < String > , pub source_file : PathBuf , pub crate_name : String , pub resolved_dependencies : HashSet < String > , pub is_proc_macro : bool , pub required_imports : HashSet < String > , pub direct_dependencies : HashSet < String > , pub extern_crates : HashSet < String > , pub is_public : bool , }
    };
}

SerializableDeclaration!();
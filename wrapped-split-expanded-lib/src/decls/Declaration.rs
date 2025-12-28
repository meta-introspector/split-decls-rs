macro_rules! deps {
    () => {
        DeclarationItem!();
    };
}

macro_rules! Declaration {
    () => {
        deps!();
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct Declaration { pub item : DeclarationItem , pub referenced_types : HashSet < String > , pub referenced_functions : HashSet < String > , pub external_identifiers : HashSet < String > , pub source_file : PathBuf , pub crate_name : String , pub resolved_dependencies : HashSet < String > , pub is_proc_macro : bool , pub required_imports : HashSet < String > , pub direct_dependencies : HashSet < String > , pub extern_crates : HashSet < String > , pub is_public : bool , }
    };
}

Declaration!()
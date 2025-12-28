macro_rules! ResolvedDependency {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub struct ResolvedDependency { pub id : String , pub dependency_type : String , pub crate_name : String , pub module_path : String , pub usage_count : usize , }
    };
}

ResolvedDependency!();
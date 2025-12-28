macro_rules! deps {
    () => {
        Dependency!();
    };
}

macro_rules! TargetDependencies {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Clone , Debug)] pub (crate) struct TargetDependencies { # [serde (default , skip_serializing_if = "Map::is_empty")] pub dependencies : Map < String , Dependency > , # [serde (default , alias = "dev-dependencies" , skip_serializing_if = "Map::is_empty")] pub dev_dependencies : Map < String , Dependency > , }
    };
}

TargetDependencies!();
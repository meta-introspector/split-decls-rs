macro_rules! deps {
    () => {
        Patch!();
    };
}

macro_rules! RegistryPatch {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Clone , Debug)] # [serde (transparent)] pub (crate) struct RegistryPatch { pub crates : Map < String , Patch > , }
    };
}

RegistryPatch!();
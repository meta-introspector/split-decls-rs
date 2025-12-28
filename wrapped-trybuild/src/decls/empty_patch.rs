macro_rules! deps {
    () => {
        RegistryPatch!();
    };
}

macro_rules! empty_patch {
    () => {
        deps!();
        fn empty_patch (patch : & Map < String , RegistryPatch >) -> bool { patch . values () . all (| registry_patch | registry_patch . crates . is_empty ()) }
    };
}

empty_patch!()
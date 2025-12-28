macro_rules! deps {
    () => {
        Result!();
        Error!();
        RegistryPatch!();
    };
}

macro_rules! serialize_patch {
    () => {
        deps!();
        fn serialize_patch < S > (patch : & Map < String , RegistryPatch > , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut map = serializer . serialize_map (None) ? ; for (registry , patch) in patch { if ! patch . crates . is_empty () { map . serialize_entry (registry , patch) ? ; } } map . end () }
    };
}

serialize_patch!()
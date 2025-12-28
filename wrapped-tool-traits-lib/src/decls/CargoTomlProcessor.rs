macro_rules! deps {
    () => {
        RepoState!();
    };
}

macro_rules! CargoTomlProcessor {
    () => {
        deps!();
        pub trait CargoTomlProcessor : Send + Sync { fn process_cargo_toml (& self , path : & Path , discovered_repos : & mut HashSet < RepoState > , all_vendored_crate_names : & mut HashSet < String > , target_org : & str , target_branch : & str ,) -> std :: result :: Result < () , String > ; }
    };
}

CargoTomlProcessor!()
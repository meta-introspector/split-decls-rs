macro_rules! CargoMetadataProvider {
    () => {
        pub trait CargoMetadataProvider : Send + Sync { fn provide_metadata (& self , manifest_path : & Path) -> Result < Metadata > ; }
    };
}

CargoMetadataProvider!()
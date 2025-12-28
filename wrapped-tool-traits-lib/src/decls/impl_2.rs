macro_rules! deps {
    () => {
        DummyCargoMetadataProvider!();
        CargoMetadataProvider!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl CargoMetadataProvider for DummyCargoMetadataProvider { fn provide_metadata (& self , _manifest_path : & Path) -> Result < Metadata > { anyhow :: bail ! ("`CargoMetadataProvider` requires the `cargo_metadata_enabled` feature to be enabled.") } }
    };
}

impl_2!();
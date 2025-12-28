macro_rules! deps {
    () => {
        CargoMetadataProvider!();
        RealCargoMetadataProvider!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl CargoMetadataProvider for RealCargoMetadataProvider { fn provide_metadata (& self , manifest_path : & Path) -> Result < Metadata > { cargo_metadata :: MetadataCommand :: new () . manifest_path (manifest_path) . exec () . map_err (| e | anyhow :: anyhow ! ("Failed to get cargo metadata: {}" , e)) } }
    };
}

impl_4!();
macro_rules! deps {
    () => {
        FileNameEmbeddablePreference!();
        FileNameDisplayPreference!();
    };
}

macro_rules! FilePathMapping {
    () => {
        deps!();
        # [derive (Clone)] pub struct FilePathMapping { mapping : Vec < (PathBuf , PathBuf) > , filename_display_for_diagnostics : FileNameDisplayPreference , filename_embeddable_preference : FileNameEmbeddablePreference , }
    };
}

FilePathMapping!();
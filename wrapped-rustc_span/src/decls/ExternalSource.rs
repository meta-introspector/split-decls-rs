macro_rules! deps {
    () => {
        ExternalSourceKind!();
        SourceFile!();
    };
}

macro_rules! ExternalSource {
    () => {
        deps!();
        # [derive (PartialEq , Eq , Clone , Debug)] pub enum ExternalSource { # [doc = " No external source has to be loaded, since the `SourceFile` represents a local crate."] Unneeded , Foreign { kind : ExternalSourceKind , # [doc = " Index of the file inside metadata."] metadata_index : u32 , } , }
    };
}

ExternalSource!();
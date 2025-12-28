macro_rules! deps {
    () => {
        Alignment!();
    };
}

macro_rules! MetadataCastError {
    () => {
        deps!();
        # [cfg_attr (test , derive (Debug))] pub (crate) enum MetadataCastError { Alignment , Size , }
    };
}

MetadataCastError!();
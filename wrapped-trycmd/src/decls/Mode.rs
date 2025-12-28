macro_rules! Mode {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum Mode { Fail , Overwrite , Dump (std :: path :: PathBuf) , }
    };
}

Mode!();
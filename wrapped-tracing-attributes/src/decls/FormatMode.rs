macro_rules! FormatMode {
    () => {
        # [derive (Clone , Debug , Hash , PartialEq , Eq , Default)] pub (crate) enum FormatMode { # [default] Default , Display , Debug , }
    };
}

FormatMode!();
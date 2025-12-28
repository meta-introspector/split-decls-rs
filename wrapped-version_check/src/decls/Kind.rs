macro_rules! Kind {
    () => {
        # [derive (Debug , PartialEq , Eq , Copy , Clone)] enum Kind { Dev , Nightly , Beta , Stable , }
    };
}

Kind!();
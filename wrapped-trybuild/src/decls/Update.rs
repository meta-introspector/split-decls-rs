macro_rules! Update {
    () => {
        # [derive (PartialEq , Default , Debug)] pub (crate) enum Update { # [default] Wip , Overwrite , }
    };
}

Update!()
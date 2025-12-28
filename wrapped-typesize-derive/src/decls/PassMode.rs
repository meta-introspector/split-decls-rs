macro_rules! PassMode {
    () => {
        # [derive (Clone , Copy)] pub (crate) enum PassMode { AsIs , InsertRef , Packed , }
    };
}

PassMode!();
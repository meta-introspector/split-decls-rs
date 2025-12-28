macro_rules! Flavor {
    () => {
        # [derive (PartialEq)] pub (crate) enum Flavor { General , FastcallOrVectorcall , }
    };
}

Flavor!()
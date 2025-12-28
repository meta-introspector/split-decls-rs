macro_rules! Uninhabited {
    () => {
        # [derive (Debug)] pub (crate) struct Uninhabited ;
    };
}

Uninhabited!();
macro_rules! PersistOptions {
    () => {
        # [derive (Debug , Default , Clone)] pub struct PersistOptions { # [doc = " Path to a custom serialize function."] pub serialize_fn : Option < syn :: Path > , # [doc = " Path to a custom serialize function."] pub deserialize_fn : Option < syn :: Path > , }
    };
}

PersistOptions!()
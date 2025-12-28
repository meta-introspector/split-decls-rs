macro_rules! macro_128 {
    () => {
        rustc_index :: newtype_index ! { # [orderable] # [gate_rustc_only] pub (super) struct StackDepth { } }
    };
}

macro_128!()
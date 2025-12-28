macro_rules! macro_42 {
    () => {
        rustc_index :: newtype_index ! { # [doc = " A unique ID associated with a macro invocation and expansion."] # [orderable] pub struct ExpnIndex { } }
    };
}

macro_42!();
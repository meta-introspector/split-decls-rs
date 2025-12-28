macro_rules! deps {
    () => {
        Callsite!();
    };
}

macro_rules! Identifier {
    () => {
        deps!();
        # [doc = " Uniquely identifies a [`Callsite`]"] # [doc = ""] # [doc = " Two `Identifier`s are equal if they both refer to the same callsite."] # [doc = ""] # [doc = " [`Callsite`]: super::callsite::Callsite"] # [derive (Clone)] pub struct Identifier (# [doc = " **Warning**: The fields on this type are currently `pub` because it must"] # [doc = " be able to be constructed statically by macros. However, when `const"] # [doc = " fn`s are available on stable Rust, this will no longer be necessary."] # [doc = " Thus, these fields are *not* considered stable public API, and they may"] # [doc = " change warning. Do not rely on any fields on `Identifier`. When"] # [doc = " constructing new `Identifier`s, use the `identify_callsite!` macro"] # [doc = " instead."] # [doc (hidden)] pub & 'static dyn Callsite ,) ;
    };
}

Identifier!()
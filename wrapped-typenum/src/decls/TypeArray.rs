macro_rules! TypeArray {
    () => {
        # [doc = " The **marker trait** for type-level arrays of type-level numbers."] # [doc = ""] # [doc = " Someday, it may contain an associated constant to produce a runtime array,"] # [doc = " like the other marker traits here. However, that is blocked by [this"] # [doc = " issue](https://github.com/rust-lang/rust/issues/44168)."] pub trait TypeArray : Sealed { }
    };
}

TypeArray!()
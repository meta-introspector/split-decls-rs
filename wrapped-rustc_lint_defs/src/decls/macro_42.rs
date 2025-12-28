macro_rules! macro_42 {
    () => {
        declare_lint ! { # [doc = " The `unused_unsafe` lint detects unnecessary use of an `unsafe` block."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " unsafe {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " If nothing within the block requires `unsafe`, then remove the"] # [doc = " `unsafe` marker because it is not required and may cause confusion."] pub UNUSED_UNSAFE , Warn , "unnecessary use of an `unsafe` block" }
    };
}

macro_42!()
macro_rules! macro_9 {
    () => {
        declare_lint ! { # [doc = " The `redundant_imports` lint detects imports that are redundant due to being"] # [doc = " imported already; either through a previous import, or being present in"] # [doc = " the prelude."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(redundant_imports)]"] # [doc = " use std::option::Option::None;"] # [doc = " fn foo() -> Option<i32> { None }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Redundant imports are unnecessary and can be removed to simplify code."] # [doc = " If you intended to re-export the item to make it available outside of the"] # [doc = " module, add a visibility modifier like `pub`."] pub REDUNDANT_IMPORTS , Allow , "imports that are redundant due to being imported already" }
    };
}

macro_9!()
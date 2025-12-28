macro_rules! macro_8 {
    () => {
        declare_lint ! { # [doc = " The `unused_imports` lint detects imports that are never used."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::collections::HashMap;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused imports may signal a mistake or unfinished code, and clutter"] # [doc = " the code, and should be removed. If you intended to re-export the item"] # [doc = " to make it available outside of the module, add a visibility modifier"] # [doc = " like `pub`."] pub UNUSED_IMPORTS , Warn , "imports that are never used" }
    };
}

macro_8!()
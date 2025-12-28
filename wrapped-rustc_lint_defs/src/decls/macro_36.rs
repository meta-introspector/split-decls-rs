macro_rules! macro_36 {
    () => {
        declare_lint ! { # [doc = " The `renamed_and_removed_lints` lint detects lints that have been"] # [doc = " renamed or removed."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![deny(raw_pointer_derive)]"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " To fix this, either remove the lint or use the new name. This can help"] # [doc = " avoid confusion about lints that are no longer valid, and help"] # [doc = " maintain consistency for renamed lints."] pub RENAMED_AND_REMOVED_LINTS , Warn , "lints that have been renamed or removed" }
    };
}

macro_36!();
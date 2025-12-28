macro_rules! macro_16 {
    () => {
        declare_lint ! { # [doc = " The `unused_variables` lint detects variables which are not used in"] # [doc = " any way."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let x = 5;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Unused variables may signal a mistake or unfinished code. To silence"] # [doc = " the warning for the individual variable, prefix it with an underscore"] # [doc = " such as `_x`."] pub UNUSED_VARIABLES , Warn , "detect variables which are not used in any way" }
    };
}

macro_16!();
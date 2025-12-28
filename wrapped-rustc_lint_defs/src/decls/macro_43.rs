macro_rules! macro_43 {
    () => {
        declare_lint ! { # [doc = " The `unused_mut` lint detects mut variables which don't need to be"] # [doc = " mutable."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let mut x = 5;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The preferred style is to only mark variables as `mut` if it is"] # [doc = " required."] pub UNUSED_MUT , Warn , "detect mut variables which don't need to be mutable" }
    };
}

macro_43!();
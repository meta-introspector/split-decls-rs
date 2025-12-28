macro_rules! macro_14 {
    () => {
        declare_lint ! { # [doc = " The `unknown_lints` lint detects unrecognized lint attributes."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![allow(not_a_real_lint)]"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is usually a mistake to specify a lint that does not exist. Check"] # [doc = " the spelling, and check the lint listing for the correct name. Also"] # [doc = " consider if you are using an old version of the compiler, and the lint"] # [doc = " is only available in a newer version."] pub UNKNOWN_LINTS , Warn , "unrecognized lint attribute" , @ eval_always = true }
    };
}

macro_14!();
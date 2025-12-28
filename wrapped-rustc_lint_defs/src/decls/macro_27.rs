macro_rules! macro_27 {
    () => {
        declare_lint ! { # [doc = " The `warnings` lint allows you to change the level of other"] # [doc = " lints which produce warnings."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![deny(warnings)]"] # [doc = " fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " The `warnings` lint is a bit special; by changing its level, you"] # [doc = " change every other warning that would produce a warning to whatever"] # [doc = " value you'd like. As such, you won't ever trigger this lint in your"] # [doc = " code directly."] pub WARNINGS , Warn , "mass-change the level for lints which produce warnings" }
    };
}

macro_27!()
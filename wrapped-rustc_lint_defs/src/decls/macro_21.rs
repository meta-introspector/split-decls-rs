macro_rules! macro_21 {
    () => {
        declare_lint ! { # [doc = " The `unreachable_patterns` lint detects unreachable patterns."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let x = 5;"] # [doc = " match x {"] # [doc = "     y => (),"] # [doc = "     5 => (),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This usually indicates a mistake in how the patterns are specified or"] # [doc = " ordered. In this example, the `y` pattern will always match, so the"] # [doc = " five is impossible to reach. Remember, match arms match in order, you"] # [doc = " probably wanted to put the `5` case above the `y` case."] pub UNREACHABLE_PATTERNS , Warn , "detects unreachable patterns" }
    };
}

macro_21!();
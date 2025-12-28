macro_rules! macro_23 {
    () => {
        declare_lint ! { # [doc = " The `non_contiguous_range_endpoints` lint detects likely off-by-one errors when using"] # [doc = " exclusive [range patterns]."] # [doc = ""] # [doc = " [range patterns]: https://doc.rust-lang.org/nightly/reference/patterns.html#range-patterns"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let x = 123u32;"] # [doc = " match x {"] # [doc = "     0..100 => { println!(\"small\"); }"] # [doc = "     101..1000 => { println!(\"large\"); }"] # [doc = "     _ => { println!(\"larger\"); }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is likely a mistake to have range patterns in a match expression that miss out a single"] # [doc = " number. Check that the beginning and end values are what you expect, and keep in mind that"] # [doc = " with `..=` the right bound is inclusive, and with `..` it is exclusive."] pub NON_CONTIGUOUS_RANGE_ENDPOINTS , Warn , "detects off-by-one errors with exclusive range patterns" }
    };
}

macro_23!();
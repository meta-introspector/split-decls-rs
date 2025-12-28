macro_rules! macro_22 {
    () => {
        declare_lint ! { # [doc = " The `overlapping_range_endpoints` lint detects `match` arms that have [range patterns] that"] # [doc = " overlap on their endpoints."] # [doc = ""] # [doc = " [range patterns]: https://doc.rust-lang.org/nightly/reference/patterns.html#range-patterns"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let x = 123u8;"] # [doc = " match x {"] # [doc = "     0..=100 => { println!(\"small\"); }"] # [doc = "     100..=255 => { println!(\"large\"); }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is likely a mistake to have range patterns in a match expression that overlap in this"] # [doc = " way. Check that the beginning and end values are what you expect, and keep in mind that"] # [doc = " with `..=` the left and right bounds are inclusive."] pub OVERLAPPING_RANGE_ENDPOINTS , Warn , "detects range patterns with overlapping endpoints" }
    };
}

macro_22!();
macro_rules! macro_6 {
    () => {
        declare_lint ! { # [doc = " The `arithmetic_overflow` lint detects that an arithmetic operation"] # [doc = " will [overflow]."] # [doc = ""] # [doc = " [overflow]: https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow"] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " 1_i32 << 32;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " It is very likely a mistake to perform an arithmetic operation that"] # [doc = " overflows its value. If the compiler is able to detect these kinds of"] # [doc = " overflows at compile-time, it will trigger this lint. Consider"] # [doc = " adjusting the expression to avoid overflow, or use a data type that"] # [doc = " will not overflow."] pub ARITHMETIC_OVERFLOW , Deny , "arithmetic operation overflows" , @ eval_always = true }
    };
}

macro_6!();
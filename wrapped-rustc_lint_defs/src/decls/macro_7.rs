macro_rules! macro_7 {
    () => {
        declare_lint ! { # [doc = " The `unconditional_panic` lint detects an operation that will cause a"] # [doc = " panic at runtime."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " # #![allow(unused)]"] # [doc = " let x = 1 / 0;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " This lint detects code that is very likely incorrect because it will"] # [doc = " always panic, such as division by zero and out-of-bounds array"] # [doc = " accesses. Consider adjusting your code if this is a bug, or using the"] # [doc = " `panic!` or `unreachable!` macro instead in case the panic is intended."] pub UNCONDITIONAL_PANIC , Deny , "operation will cause a panic at runtime" , @ eval_always = true }
    };
}

macro_7!()
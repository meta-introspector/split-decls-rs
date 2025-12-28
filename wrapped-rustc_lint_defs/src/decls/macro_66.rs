macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_66 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `unsafe_op_in_unsafe_fn` lint detects unsafe operations in unsafe"] # [doc = " functions without an explicit unsafe block."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,compile_fail"] # [doc = " #![deny(unsafe_op_in_unsafe_fn)]"] # [doc = ""] # [doc = " unsafe fn foo() {}"] # [doc = ""] # [doc = " unsafe fn bar() {"] # [doc = "     foo();"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Currently, an [`unsafe fn`] allows any [unsafe] operation within its"] # [doc = " body. However, this can increase the surface area of code that needs"] # [doc = " to be scrutinized for proper behavior. The [`unsafe` block] provides a"] # [doc = " convenient way to make it clear exactly which parts of the code are"] # [doc = " performing unsafe operations. In the future, it is desired to change"] # [doc = " it so that unsafe operations cannot be performed in an `unsafe fn`"] # [doc = " without an `unsafe` block."] # [doc = ""] # [doc = " The fix to this is to wrap the unsafe code in an `unsafe` block."] # [doc = ""] # [doc = " This lint is \"allow\" by default on editions up to 2021, from 2024 it is"] # [doc = " \"warn\" by default; the plan for increasing severity further is"] # [doc = " still being considered. See [RFC #2585] and [issue #71668] for more"] # [doc = " details."] # [doc = ""] # [doc = " [`unsafe fn`]: https://doc.rust-lang.org/reference/unsafe-functions.html"] # [doc = " [`unsafe` block]: https://doc.rust-lang.org/reference/expressions/block-expr.html#unsafe-blocks"] # [doc = " [unsafe]: https://doc.rust-lang.org/reference/unsafety.html"] # [doc = " [RFC #2585]: https://github.com/rust-lang/rfcs/blob/master/text/2585-unsafe-block-in-unsafe-fn.md"] # [doc = " [issue #71668]: https://github.com/rust-lang/rust/issues/71668"] pub UNSAFE_OP_IN_UNSAFE_FN , Allow , "unsafe operations in unsafe functions without an explicit unsafe block are deprecated" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionSemanticsChange (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html>" , explain_reason : false } ; @ edition Edition2024 => Warn ; }
    };
}

macro_66!()
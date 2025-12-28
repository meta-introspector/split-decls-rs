macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
        FutureIncompatibleInfo!();
    };
}

macro_rules! macro_125 {
    () => {
        deps!();
        declare_lint ! { # [doc = " The `missing_unsafe_on_extern` lint detects missing unsafe keyword on extern declarations."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,edition2021"] # [doc = " #![warn(missing_unsafe_on_extern)]"] # [doc = " #![allow(dead_code)]"] # [doc = ""] # [doc = " extern \"C\" {"] # [doc = "     fn foo(_: i32);"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Declaring extern items, even without ever using them, can cause Undefined Behavior. We"] # [doc = " should consider all sources of Undefined Behavior to be unsafe."] # [doc = ""] # [doc = " This is a [future-incompatible] lint to transition this to a"] # [doc = " hard error in the future."] # [doc = ""] # [doc = " [future-incompatible]: ../index.md#future-incompatible-lints"] pub MISSING_UNSAFE_ON_EXTERN , Allow , "detects missing unsafe keyword on extern declarations" , @ future_incompatible = FutureIncompatibleInfo { reason : FutureIncompatibilityReason :: EditionError (Edition :: Edition2024) , reference : "<https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-extern.html>" , } ; }
    };
}

macro_125!()
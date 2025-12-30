// Generated macro for autodiff (module)
macro_rules! Depcrateautodiff {
() => {
// Module: crate
// Provides: {"autodiff"}
// Dependencies: {}
# [unstable (feature = "autodiff" , issue = "124509")] # [doc = " This module provides support for automatic differentiation."] pub mod autodiff { # [doc = " This macro handles automatic differentiation."] pub use core :: autodiff :: { autodiff_forward , autodiff_reverse } ; }
};
}

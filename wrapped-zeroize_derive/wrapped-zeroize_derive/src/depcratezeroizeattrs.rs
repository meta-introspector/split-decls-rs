// Generated macro for ZeroizeAttrs (struct)
macro_rules! DepcrateZeroizeAttrs {
() => {
// Module: crate
// Provides: {"ZeroizeAttrs"}
// Dependencies: {}
# [doc = " Custom derive attributes for `Zeroize`"] # [derive (Default)] struct ZeroizeAttrs { # [doc = " Derive a `Drop` impl which calls zeroize on this type"] drop : bool , # [doc = " Custom bounds as defined by the user"] bound : Option < Bounds > , # [doc = " Type parameters in use by fields"] auto_params : Vec < Ident > , }
};
}

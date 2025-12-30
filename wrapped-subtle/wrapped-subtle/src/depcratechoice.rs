// Generated macro for Choice (struct)
macro_rules! DepcrateChoice {
() => {
// Module: crate
// Provides: {"Choice"}
// Dependencies: {}
# [doc = " The `Choice` struct represents a choice for use in conditional assignment."] # [doc = ""] # [doc = " It is a wrapper around a `u8`, which should have the value either `1` (true)"] # [doc = " or `0` (false)."] # [doc = ""] # [doc = " The conversion from `u8` to `Choice` passes the value through an optimization"] # [doc = " barrier, as a best-effort attempt to prevent the compiler from inferring that"] # [doc = " the `Choice` value is a boolean. This strategy is based on Tim Maclean's"] # [doc = " [work on `rust-timing-shield`][rust-timing-shield], which attempts to provide"] # [doc = " a more comprehensive approach for preventing software side-channels in Rust"] # [doc = " code."] # [doc = ""] # [doc = " The `Choice` struct implements operators for AND, OR, XOR, and NOT, to allow"] # [doc = " combining `Choice` values. These operations do not short-circuit."] # [doc = ""] # [doc = " [rust-timing-shield]:"] # [doc = " https://www.chosenplaintext.ca/open-source/rust-timing-shield/security"] # [derive (Copy , Clone , Debug)] pub struct Choice (u8) ;
};
}

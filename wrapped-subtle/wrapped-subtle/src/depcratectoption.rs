// Generated macro for CtOption (struct)
macro_rules! DepcrateCtOption {
() => {
// Module: crate
// Provides: {"CtOption"}
// Dependencies: {}
# [doc = " The `CtOption<T>` type represents an optional value similar to the"] # [doc = " [`Option<T>`](core::option::Option) type but is intended for"] # [doc = " use in constant time APIs."] # [doc = ""] # [doc = " Any given `CtOption<T>` is either `Some` or `None`, but unlike"] # [doc = " `Option<T>` these variants are not exposed. The"] # [doc = " [`is_some()`](CtOption::is_some) method is used to determine if"] # [doc = " the value is `Some`, and [`unwrap_or()`](CtOption::unwrap_or) and"] # [doc = " [`unwrap_or_else()`](CtOption::unwrap_or_else) methods are"] # [doc = " provided to access the underlying value. The value can also be"] # [doc = " obtained with [`unwrap()`](CtOption::unwrap) but this will panic"] # [doc = " if it is `None`."] # [doc = ""] # [doc = " Functions that are intended to be constant time may not produce"] # [doc = " valid results for all inputs, such as square root and inversion"] # [doc = " operations in finite field arithmetic. Returning an `Option<T>`"] # [doc = " from these functions makes it difficult for the caller to reason"] # [doc = " about the result in constant time, and returning an incorrect"] # [doc = " value burdens the caller and increases the chance of bugs."] # [derive (Clone , Copy , Debug)] pub struct CtOption < T > { value : T , is_some : Choice , }
};
}

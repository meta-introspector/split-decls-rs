// Generated macro for zerovec (macro)
macro_rules! Depcrate_zeroveczerovec {
() => {
// Module: crate::zerovec
// Provides: {"zerovec"}
// Dependencies: {}
# [doc = " Creates a borrowed `ZeroVec`. Convenience wrapper for `zeroslice!(...).as_zerovec()`. The value"] # [doc = " will be created at compile-time, meaning that all arguments must also be constant."] # [doc = ""] # [doc = " See [`zeroslice!`](crate::zeroslice) for more information."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use zerovec::{ZeroVec, zerovec, ule::AsULE};"] # [doc = ""] # [doc = " const SIGNATURE: ZeroVec<char> = zerovec!(char; <char as AsULE>::ULE::from_aligned; ['a', 'y', 'e', '✌']);"] # [doc = " assert!(!SIGNATURE.is_owned());"] # [doc = ""] # [doc = " const EMPTY: ZeroVec<u32> = zerovec![];"] # [doc = " assert!(!EMPTY.is_owned());"] # [doc = " ```"] # [macro_export] macro_rules ! zerovec { () => ($ crate :: ZeroVec :: new ()) ; ($ aligned : ty ; $ convert : expr ; [$ ($ x : expr) ,+ $ (,) ?]) => ($ crate :: zeroslice ! [$ aligned ; $ convert ; [$ ($ x) ,+]] . as_zerovec ()) ; }
};
}

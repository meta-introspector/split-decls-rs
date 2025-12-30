// Generated macro for union_padding (macro)
macro_rules! Depcrate_util_macro_utilunion_padding {
() => {
// Module: crate::util::macro_util
// Provides: {"union_padding"}
// Dependencies: {}
# [doc = " Does the union type `$t` have padding?"] # [doc = ""] # [doc = " `$ts` is the list of the type of every field in `$t`. `$t` must be a union"] # [doc = " type, or else `union_padding!`'s result may be meaningless."] # [doc = ""] # [doc = " Note that `union_padding!`'s results are independent of `repr` since they"] # [doc = " only consider the size of the type and the sizes of the fields. Whatever the"] # [doc = " repr, the size of the type already takes into account any padding that the"] # [doc = " compiler has decided to add. Unions with well-defined representations (such"] # [doc = " as `repr(C)`) can use this macro to check for padding. Note that while this"] # [doc = " may yield some consistent value for some `repr(Rust)` unions, it is not"] # [doc = " guaranteed across platforms or compilations."] # [doc (hidden)] # [macro_export] macro_rules ! union_padding { ($ t : ty , [$ ($ ts : ty) ,*]) => { { let mut max = 0 ; $ ({ let padding = $ crate :: util :: macro_util :: size_of ::<$ t > () - $ crate :: util :: macro_util :: size_of ::<$ ts > () ; if padding > max { max = padding ; } }) * max } } ; }
};
}

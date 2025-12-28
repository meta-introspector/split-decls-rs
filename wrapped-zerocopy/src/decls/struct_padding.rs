macro_rules! struct_padding {
    () => {
        # [doc = " How many padding bytes does the struct type `$t` have?"] # [doc = ""] # [doc = " `$ts` is the list of the type of every field in `$t`. `$t` must be a struct"] # [doc = " type, or else `struct_padding!`'s result may be meaningless."] # [doc = ""] # [doc = " Note that `struct_padding!`'s results are independent of `repcr` since they"] # [doc = " only consider the size of the type and the sizes of the fields. Whatever the"] # [doc = " repr, the size of the type already takes into account any padding that the"] # [doc = " compiler has decided to add. Structs with well-defined representations (such"] # [doc = " as `repr(C)`) can use this macro to check for padding. Note that while this"] # [doc = " may yield some consistent value for some `repr(Rust)` structs, it is not"] # [doc = " guaranteed across platforms or compilations."] # [doc (hidden)] # [macro_export] macro_rules ! struct_padding { ($ t : ty , [$ ($ ts : ty) ,*]) => { $ crate :: util :: macro_util :: size_of ::<$ t > () - (0 $ (+ $ crate :: util :: macro_util :: size_of ::<$ ts > ()) *) } ; }
    };
}

struct_padding!();
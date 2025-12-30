// Generated macro for VariantArray (trait)
macro_rules! DepcrateVariantArray {
() => {
// Module: crate
// Provides: {"VariantArray"}
// Dependencies: {}
# [doc = " A trait for retrieving a static array containing all the variants in an Enum."] # [doc = " This trait can be autoderived by `strum_macros`. For derived usage, all the"] # [doc = " variants in the enumerator need to be unit-types, which means you can't autoderive"] # [doc = " enums with inner data in one or more variants. Consider using it alongside"] # [doc = " [`EnumDiscriminants`] if you require inner data but still want to have an"] # [doc = " static array of variants."] pub trait VariantArray : :: core :: marker :: Sized + 'static { const VARIANTS : & 'static [Self] ; }
};
}

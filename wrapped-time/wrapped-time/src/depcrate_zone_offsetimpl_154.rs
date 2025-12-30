// Generated macro for impl_154 (impl)
macro_rules! Depcrate_zone_offsetimpl_154 {
() => {
// Module: crate::zone::offset
// Provides: {"impl_154"}
// Dependencies: {}
# [allow (deprecated)] impl VariantOffsetsCalculatorBorrowed < 'static > { # [doc = " Constructs a `VariantOffsetsCalculatorBorrowed` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [inline] pub const fn new () -> Self { Self { offset_period : OffsetDataBorrowed :: New (crate :: provider :: Baked :: SINGLETON_TIMEZONE_PERIODS_V1 ,) , } } # [doc = " Cheaply converts a [`VariantOffsetsCalculatorBorrowed<'static>`] into a [`VariantOffsetsCalculator`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`VariantOffsetsCalculator`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`VariantOffsetsCalculatorBorrowed`]."] pub fn static_to_owned (& self) -> VariantOffsetsCalculator { VariantOffsetsCalculator { offset_period : match self . offset_period { OffsetDataBorrowed :: New (p) => OffsetData :: New (DataPayload :: from_static_ref (p)) , # [cfg (feature = "alloc")] OffsetDataBorrowed :: Old (p) => OffsetData :: Old (DataPayload :: from_static_ref (p)) , } , } } }
};
}

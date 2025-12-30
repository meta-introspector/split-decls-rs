// Generated macro for impl_152 (impl)
macro_rules! Depcrate_graphemeimpl_152 {
() => {
// Module: crate::grapheme
// Provides: {"impl_152"}
// Dependencies: {}
impl GraphemeClusterSegmenter { # [doc = " Constructs a [`GraphemeClusterSegmenterBorrowed`] with an invariant locale from compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] # [expect (clippy :: new_ret_no_self)] pub const fn new () -> GraphemeClusterSegmenterBorrowed < 'static > { GraphemeClusterSegmenterBorrowed { data : crate :: provider :: Baked :: SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V1 , } } icu_provider :: gen_buffer_data_constructors ! (() -> error : DataError , functions : [new : skip , try_new_with_buffer_provider , try_new_unstable , Self ,]) ; # [doc = icu_provider :: gen_buffer_unstable_docs ! (UNSTABLE , Self :: new)] pub fn try_new_unstable < D > (provider : & D) -> Result < Self , DataError > where D : DataProvider < SegmenterBreakGraphemeClusterV1 > + ? Sized , { let payload = provider . load (Default :: default ()) ? . payload ; Ok (Self { payload }) } # [doc = " Constructs a borrowed version of this type for more efficient querying."] # [doc = ""] # [doc = " Most useful methods for segmentation are on this type."] pub fn as_borrowed (& self) -> GraphemeClusterSegmenterBorrowed < '_ > { GraphemeClusterSegmenterBorrowed { data : self . payload . get () , } } }
};
}

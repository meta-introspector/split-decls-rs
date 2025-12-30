// Generated macro for _ (const)
macro_rules! Depcrate_provider_ {
() => {
// Module: crate::provider
// Provides: {"_"}
// Dependencies: {}
# [cfg (feature = "compiled_data")] # [allow (unused_imports)] const _ : () = { use icu_segmenter_data :: * ; pub mod icu { pub use crate as segmenter ; pub use icu_collections as collections ; pub use icu_locale as locale ; } make_provider ! (Baked) ; impl_segmenter_break_sentence_v1 ! (Baked) ; impl_segmenter_dictionary_auto_v1 ! (Baked) ; impl_segmenter_break_grapheme_cluster_v1 ! (Baked) ; impl_segmenter_dictionary_extended_v1 ! (Baked) ; impl_segmenter_break_line_v1 ! (Baked) ; # [cfg (feature = "lstm")] impl_segmenter_lstm_auto_v1 ! (Baked) ; impl_segmenter_break_word_v1 ! (Baked) ; impl_segmenter_break_word_override_v1 ! (Baked) ; impl_segmenter_break_sentence_override_v1 ! (Baked) ; } ;
};
}

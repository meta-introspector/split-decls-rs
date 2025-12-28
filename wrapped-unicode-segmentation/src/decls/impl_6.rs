macro_rules! deps {
    () => {
        UnicodeSegmentation!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl UnicodeSegmentation for str { # [inline] fn graphemes (& self , is_extended : bool) -> Graphemes < '_ > { grapheme :: new_graphemes (self , is_extended) } # [inline] fn grapheme_indices (& self , is_extended : bool) -> GraphemeIndices { grapheme :: new_grapheme_indices (self , is_extended) } # [inline] fn unicode_words (& self) -> UnicodeWords < '_ > { word :: new_unicode_words (self) } # [inline] fn unicode_word_indices (& self) -> UnicodeWordIndices < '_ > { word :: new_unicode_word_indices (self) } # [inline] fn split_word_bounds (& self) -> UWordBounds < '_ > { word :: new_word_bounds (self) } # [inline] fn split_word_bound_indices (& self) -> UWordBoundIndices < '_ > { word :: new_word_bound_indices (self) } # [inline] fn unicode_sentences (& self) -> UnicodeSentences < '_ > { sentence :: new_unicode_sentences (self) } # [inline] fn split_sentence_bounds (& self) -> USentenceBounds < '_ > { sentence :: new_sentence_bounds (self) } # [inline] fn split_sentence_bound_indices (& self) -> USentenceBoundIndices { sentence :: new_sentence_bound_indices (self) } }
    };
}

impl_6!()
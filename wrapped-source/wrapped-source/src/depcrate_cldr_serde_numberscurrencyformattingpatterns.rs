// Generated macro for CurrencyFormattingPatterns (struct)
macro_rules! Depcrate_cldr_serde_numbersCurrencyFormattingPatterns {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"CurrencyFormattingPatterns"}
// Dependencies: {}
# [derive (PartialEq , Debug , Deserialize)] pub (crate) struct CurrencyFormattingPatterns { # [doc = " Standard pattern"] pub (crate) standard : String , # [doc = " Contains the compact currency patterns for short compact currency formatting"] # [serde (rename = "short")] pub (crate) compact_short : Option < ShortCompactCurrencyPatterns > , # [doc = " Standard alphaNextToNumber pattern"] # [serde (rename = "standard-alphaNextToNumber")] pub (crate) standard_alpha_next_to_number : Option < String > , # [serde (rename = "unitPattern-count-0")] pub (crate) pattern_explicit_zero : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-1")] pub (crate) pattern_explicit_one : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-zero")] pub (crate) pattern_zero : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-one")] pub (crate) pattern_one : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-two")] pub (crate) pattern_two : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-few")] pub (crate) pattern_few : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-many")] pub (crate) pattern_many : Option < PatternString < DoublePlaceholder > > , # [serde (rename = "unitPattern-count-other")] pub (crate) pattern_other : Option < PatternString < DoublePlaceholder > > , }
};
}

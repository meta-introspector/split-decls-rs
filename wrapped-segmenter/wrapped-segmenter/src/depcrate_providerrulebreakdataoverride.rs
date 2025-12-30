// Generated macro for RuleBreakDataOverride (struct)
macro_rules! Depcrate_providerRuleBreakDataOverride {
() => {
// Module: crate::provider
// Provides: {"RuleBreakDataOverride"}
// Dependencies: {}
# [doc = " codepoint trie data that the difference by specific locale"] # [derive (Debug , PartialEq , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake) , databake (path = icu_segmenter :: provider) ,)] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] pub struct RuleBreakDataOverride < 'data > { # [doc = " The difference of property table for special locale."] # [cfg_attr (feature = "serde" , serde (borrow))] pub property_table_override : CodePointTrie < 'data , u8 > , }
};
}

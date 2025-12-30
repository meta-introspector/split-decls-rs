// Generated macro for LetVariant (enum)
macro_rules! Depcrate_expressionLetVariant {
() => {
// Module: crate::expression
// Provides: {"LetVariant"}
// Dependencies: {}
# [derive (Debug , Clone , Serialize , Deserialize)] # [serde (untagged)] pub enum LetVariant { Basic (WildString , Box < Expression >) , WithType (WildString , TypeKind , Box < Expression >) , MutWithType (WildString , TypeKind , Box < Expression >) , }
};
}

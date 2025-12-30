// Generated macro for Answer (enum)
macro_rules! DepcrateAnswer {
() => {
// Module: crate
// Provides: {"Answer"}
// Dependencies: {}
# [doc = " Either transmutation is allowed, we have an error, or we have an optional"] # [doc = " Condition that must hold."] # [derive (Debug , Hash , Eq , PartialEq , Clone)] pub enum Answer < R , T > { Yes , No (Reason < T >) , If (Condition < R , T >) , }
};
}

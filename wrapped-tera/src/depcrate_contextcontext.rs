// Generated macro for Context (struct)
macro_rules! Depcrate_contextContext {
() => {
// Module: crate::context
// Provides: {"Context"}
// Dependencies: {}
# [doc = " The struct that holds the context of a template rendering."] # [doc = ""] # [doc = " Light wrapper around a `BTreeMap` for easier insertions of Serializable"] # [doc = " values"] # [derive (Debug , Clone , PartialEq)] pub struct Context { data : BTreeMap < String , Value > , }
};
}

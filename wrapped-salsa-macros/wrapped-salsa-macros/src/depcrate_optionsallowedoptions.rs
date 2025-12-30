// Generated macro for AllowedOptions (trait)
macro_rules! Depcrate_optionsAllowedOptions {
() => {
// Module: crate::options
// Provides: {"AllowedOptions"}
// Dependencies: {}
# [doc = " These flags determine which options are allowed in a given context"] pub (crate) trait AllowedOptions { const RETURNS : bool ; const SPECIFY : bool ; const NO_EQ : bool ; const DEBUG : bool ; const NO_LIFETIME : bool ; const NON_UPDATE_RETURN_TYPE : bool ; const SINGLETON : bool ; const DATA : bool ; const DB : bool ; const CYCLE_FN : bool ; const CYCLE_INITIAL : bool ; const CYCLE_RESULT : bool ; const LRU : bool ; const CONSTRUCTOR_NAME : bool ; const ID : bool ; const REVISIONS : bool ; const HEAP_SIZE : bool ; const SELF_TY : bool ; const PERSIST : AllowedPersistOptions ; }
};
}

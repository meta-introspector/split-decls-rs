// Generated macro for PROBLEMATIC_CONSTS_STRINGS (static)
macro_rules! Depcrate_stylePROBLEMATIC_CONSTS_STRINGS {
() => {
// Module: crate::style
// Provides: {"PROBLEMATIC_CONSTS_STRINGS"}
// Dependencies: {}
static PROBLEMATIC_CONSTS_STRINGS : LazyLock < Vec < String > > = LazyLock :: new (| | { generate_problematic_strings (ROOT_PROBLEMATIC_CONSTS , & LETTER_DIGIT . iter () . cloned () . collect ()) }) ;
};
}

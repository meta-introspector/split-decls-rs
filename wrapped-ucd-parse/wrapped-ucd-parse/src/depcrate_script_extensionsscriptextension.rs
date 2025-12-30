// Generated macro for ScriptExtension (struct)
macro_rules! Depcrate_script_extensionsScriptExtension {
() => {
// Module: crate::script_extensions
// Provides: {"ScriptExtension"}
// Dependencies: {}
# [doc = " A single row in the `ScriptExtensions.txt` file."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct ScriptExtension { # [doc = " The codepoint or codepoint range for this entry."] pub codepoints : Codepoints , # [doc = " The script extension names assigned to the codepoints in this entry."] pub scripts : Vec < String > , }
};
}

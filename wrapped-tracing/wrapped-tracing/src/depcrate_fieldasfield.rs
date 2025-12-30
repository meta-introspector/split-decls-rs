// Generated macro for AsField (trait)
macro_rules! Depcrate_fieldAsField {
() => {
// Module: crate::field
// Provides: {"AsField"}
// Dependencies: {}
# [doc = " Trait implemented to allow a type to be used as a field key."] # [doc = ""] # [doc = " <pre class=\"ignore\" style=\"white-space:normal;font:inherit;\">"] # [doc = " <strong>Note</strong>: Although this is implemented for both the"] # [doc = " <a href=\"./struct.Field.html\"><code>Field</code></a> type <em>and</em> any"] # [doc = " type that can be borrowed as an <code>&str</code>, only <code>Field</code>"] # [doc = " allows <em>O</em>(1) access."] # [doc = " Indexing a field with a string results in an iterative search that performs"] # [doc = " string comparisons. Thus, if possible, once the key for a field is known, it"] # [doc = " should be used whenever possible."] # [doc = " </pre>"] pub trait AsField : crate :: sealed :: Sealed { # [doc = " Attempts to convert `&self` into a `Field` with the specified `metadata`."] # [doc = ""] # [doc = " If `metadata` defines this field, then the field is returned. Otherwise,"] # [doc = " this returns `None`."] fn as_field (& self , metadata : & Metadata < '_ >) -> Option < Field > ; }
};
}

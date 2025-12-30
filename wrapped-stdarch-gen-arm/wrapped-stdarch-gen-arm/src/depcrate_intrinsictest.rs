// Generated macro for Test (enum)
macro_rules! Depcrate_intrinsicTest {
() => {
// Module: crate::intrinsic
// Provides: {"Test"}
// Dependencies: {}
# [doc = " Whether to generate a load/store test, and which typeset index"] # [doc = " represents the data type of the load/store target address"] # [derive (Clone , Debug , Default , Serialize , Deserialize)] # [serde (rename_all = "snake_case")] pub enum Test { # [default] # [serde (skip)] None , Load (usize) , Store (usize) , }
};
}

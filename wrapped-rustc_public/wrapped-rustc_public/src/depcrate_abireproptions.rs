// Generated macro for ReprOptions (struct)
macro_rules! Depcrate_abiReprOptions {
() => {
// Module: crate::abi
// Provides: {"ReprOptions"}
// Dependencies: {}
# [doc = " Representation options provided by the user"] # [non_exhaustive] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug , Serialize)] pub struct ReprOptions { pub int : Option < IntegerType > , pub align : Option < Align > , pub pack : Option < Align > , pub flags : ReprFlags , }
};
}

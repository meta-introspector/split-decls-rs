// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl TargetFeature { fn new (ext : & str) -> TargetFeature { match ext { "lasx" => Self :: Lasx , _ => Self :: Lsx , } } # [doc = " A string for use with `#[target_feature(...)]`."] fn as_target_feature_arg (& self , ins : & str) -> String { let vec = match * self { Self :: Lsx => "lsx" , Self :: Lasx => "lasx" , } ; let frecipe = match ins { "lsx_vfrecipe_s" | "lsx_vfrecipe_d" | "lsx_vfrsqrte_s" | "lsx_vfrsqrte_d" | "lasx_xvfrecipe_s" | "lasx_xvfrecipe_d" | "lasx_xvfrsqrte_s" | "lasx_xvfrsqrte_d" => { ",frecipe" } _ => "" , } ; format ! ("{vec}{frecipe}") } fn attr (name : & str , value : impl fmt :: Display) -> String { format ! (r#"#[{name}(enable = "{value}")]"#) } # [doc = " Generate a target_feature attribute"] fn to_target_feature_attr (self , ins : & str) -> Lines { Lines :: single (Self :: attr ("target_feature" , self . as_target_feature_arg (ins) ,)) } fn bytes (& self) -> u8 { match * self { Self :: Lsx => 16 , Self :: Lasx => 32 , } } }
};
}

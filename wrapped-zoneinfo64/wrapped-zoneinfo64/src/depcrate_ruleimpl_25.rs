// Generated macro for impl_25 (impl)
macro_rules! Depcrate_ruleimpl_25 {
() => {
// Module: crate::rule
// Provides: {"impl_25"}
// Dependencies: {}
impl TzRule { pub (crate) fn from_raw (value : & [i32 ; 11]) -> Self { Self { additional_offset_secs : value [10] , start : TzRuleDate :: new (value [1] as i8 , value [2] as i8 , value [0] as u8 , value [3] as u32 , value [4] as i8 ,) . unwrap () , end : TzRuleDate :: new (value [6] as i8 , value [7] as i8 , value [5] as u8 , value [8] as u32 , value [9] as i8 ,) . unwrap () , } } fn end_before_start (& self) -> bool { (self . start . month , self . start . day) > (self . end . month , self . end . day) } }
};
}

// Generated macro for impl_980 (impl)
macro_rules! Depcrate_configimpl_980 {
() => {
// Module: crate::config
// Provides: {"impl_980"}
// Dependencies: {}
impl de :: Visitor < '_ > for StringOrU64Visitor { type Value = u64 ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { formatter . write_str ("a string with size suffix (like '20G') or a u64") } fn visit_str < E > (self , value : & str) -> StdResult < Self :: Value , E > where E : de :: Error , { parse_size (value) . ok_or_else (| | E :: custom (format ! ("Invalid size value: {}" , value))) } fn visit_u64 < E > (self , value : u64) -> StdResult < Self :: Value , E > where E : de :: Error , { Ok (value) } fn visit_i64 < E > (self , value : i64) -> StdResult < Self :: Value , E > where E : de :: Error , { if value < 0 { Err (E :: custom ("negative values not supported")) } else { Ok (value as u64) } } }
};
}

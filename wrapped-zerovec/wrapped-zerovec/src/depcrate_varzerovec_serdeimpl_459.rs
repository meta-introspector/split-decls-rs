// Generated macro for impl_459 (impl)
macro_rules! Depcrate_varzerovec_serdeimpl_459 {
() => {
// Module: crate::varzerovec::serde
// Provides: {"impl_459"}
// Dependencies: {}
impl < 'de , T , F > Visitor < 'de > for VarZeroVecVisitor < T , F > where T : VarULE + ? Sized , F : VarZeroVecFormat , { type Value = VarZeroVec < 'de , T , F > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a sequence or borrowed buffer of bytes") } fn visit_borrowed_bytes < E > (self , bytes : & 'de [u8]) -> Result < Self :: Value , E > where E : de :: Error , { VarZeroVec :: parse_bytes (bytes) . map_err (de :: Error :: custom) } }
};
}

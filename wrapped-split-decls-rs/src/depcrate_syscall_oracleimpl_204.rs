// Generated macro for impl_204 (impl)
macro_rules! Depcrate_syscall_oracleimpl_204 {
() => {
// Module: crate::syscall_oracle
// Provides: {"impl_204"}
// Dependencies: {}
impl ToTokens for OracleType { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { let name = match self { OracleType :: FileSystem => "FileSystem" , OracleType :: Network => "Network" , OracleType :: Process => "Process" , OracleType :: Memory => "Memory" , OracleType :: Time => "Time" , OracleType :: Crypto => "Crypto" , OracleType :: Custom (s) => return s . to_tokens (tokens) , } ; tokens . extend (quote :: quote ! { # name }) ; } }
};
}

// Generated macro for impl_716 (impl)
macro_rules! Depcrate_compiler_msvcimpl_716 {
() => {
// Module: crate::compiler::msvc
// Provides: {"impl_716"}
// Dependencies: {}
impl Iterator for SplitMsvcResponseFileArgs < '_ > { type Item = String ; fn next (& mut self) -> Option < String > { let mut in_quotes = false ; let mut backslash_count : usize = 0 ; let is_whitespace = | c | matches ! (c , ' ' | '\t' | '\n' | '\r') ; self . file_content = self . file_content . trim_start_matches (is_whitespace) ; if self . file_content . is_empty () { return None ; } let mut arg = String :: new () ; let mut chars = self . file_content . chars () ; for c in & mut chars { match c { '\\' => backslash_count += 1 , '"' => { Self :: append_backslashes_to (& mut arg , & mut backslash_count , 2) ; match backslash_count == 0 { true => in_quotes = ! in_quotes , false => { backslash_count = 0 ; arg . push ('"') ; } } } ' ' | '\t' | '\n' | '\r' => { Self :: append_backslashes_to (& mut arg , & mut backslash_count , 1) ; if ! in_quotes { break ; } arg . push (c) ; } _ => { Self :: append_backslashes_to (& mut arg , & mut backslash_count , 1) ; arg . push (c) ; } } } Self :: append_backslashes_to (& mut arg , & mut backslash_count , 1) ; self . file_content = chars . as_str () ; Some (arg) } }
};
}

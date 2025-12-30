// Generated macro for impl_1256 (impl)
macro_rules! Depcrate_ioimpl_1256 {
() => {
// Module: crate::io
// Provides: {"impl_1256"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : BufRead > BufRead for Take < T > { fn fill_buf (& mut self) -> Result < & [u8] > { if self . limit == 0 { return Ok (& []) ; } let buf = self . inner . fill_buf () ? ; let cap = cmp :: min (buf . len () as u64 , self . limit) as usize ; Ok (& buf [.. cap]) } fn consume (& mut self , amt : usize) { let amt = cmp :: min (amt as u64 , self . limit) as usize ; self . limit -= amt as u64 ; self . inner . consume (amt) ; } }
};
}

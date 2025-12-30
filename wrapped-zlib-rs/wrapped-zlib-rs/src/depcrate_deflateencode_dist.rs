// Generated macro for encode_dist (function)
macro_rules! Depcrate_deflateencode_dist {
() => {
// Module: crate::deflate
// Provides: {"encode_dist"}
// Dependencies: {}
# [inline] const fn encode_dist (dtree : & [Value] , mut dist : u16) -> (u64 , usize) { dist -= 1 ; let code = State :: d_code (dist as usize) as usize ; assert ! (code < D_CODES , "bad d_code") ; let dnode = dtree [code] ; let mut match_bits = dnode . code () as u64 ; let mut match_bits_len = dnode . len () as usize ; let extra = StaticTreeDesc :: EXTRA_DBITS [code] as usize ; if extra != 0 { dist -= self :: trees_tbl :: BASE_DIST [code] ; match_bits |= (dist as u64) << match_bits_len ; match_bits_len += extra ; } (match_bits , match_bits_len) }
};
}

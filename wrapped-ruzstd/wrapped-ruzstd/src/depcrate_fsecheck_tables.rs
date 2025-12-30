// Generated macro for check_tables (function)
macro_rules! Depcrate_fsecheck_tables {
() => {
// Module: crate::fse
// Provides: {"check_tables"}
// Dependencies: {}
# [cfg (any (test , feature = "fuzz_exports"))] fn check_tables (dec_table : & fse_decoder :: FSETable , enc_table : & fse_encoder :: FSETable) { for (idx , dec_state) in dec_table . decode . iter () . enumerate () { let enc_states = & enc_table . states [dec_state . symbol as usize] ; let enc_state = enc_states . states . iter () . find (| state | state . index == idx) . unwrap () ; assert_eq ! (enc_state . baseline , dec_state . base_line as usize) ; assert_eq ! (enc_state . num_bits , dec_state . num_bits) ; } }
};
}

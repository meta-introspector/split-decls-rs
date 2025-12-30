// Generated macro for read_table_key (function)
macro_rules! Depcrate_unix_linux_systemread_table_key {
() => {
// Module: crate::unix::linux::system
// Provides: {"read_table_key"}
// Dependencies: {}
fn read_table_key (filename : & str , target_key : & str , colsep : char) -> Option < u64 > { if let Ok (content) = get_all_utf8_data (filename , 16_635) { return content . split ('\n') . find_map (| line | { let mut split = line . split (colsep) ; let key = split . next () ? ; if key != target_key { return None ; } let value = split . next () ? ; let value0 = value . trim_start () . split (' ') . next () ? ; u64 :: from_str (value0) . ok () }) ; } None }
};
}

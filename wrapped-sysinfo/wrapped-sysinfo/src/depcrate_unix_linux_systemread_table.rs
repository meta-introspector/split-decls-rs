// Generated macro for read_table (function)
macro_rules! Depcrate_unix_linux_systemread_table {
() => {
// Module: crate::unix::linux::system
// Provides: {"read_table"}
// Dependencies: {}
fn read_table < F > (filename : & str , colsep : char , mut f : F) where F : FnMut (& str , u64) , { if let Ok (content) = get_all_utf8_data (filename , 16_635) { content . split ('\n') . flat_map (| line | { let mut split = line . split (colsep) ; let key = split . next () ? ; let value = split . next () ? ; let value0 = value . trim_start () . split (' ') . next () ? ; let value0_u64 = u64 :: from_str (value0) . ok () ? ; Some ((key , value0_u64)) }) . for_each (| (k , v) | f (k , v)) ; } }
};
}

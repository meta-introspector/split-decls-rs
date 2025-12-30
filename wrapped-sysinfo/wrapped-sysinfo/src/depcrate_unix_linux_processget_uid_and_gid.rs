// Generated macro for get_uid_and_gid (function)
macro_rules! Depcrate_unix_linux_processget_uid_and_gid {
() => {
// Module: crate::unix::linux::process
// Provides: {"get_uid_and_gid"}
// Dependencies: {}
fn get_uid_and_gid (file_path : & Path) -> Option < ((uid_t , uid_t) , (gid_t , gid_t)) > { let status_data = get_all_utf8_data (file_path , 16_385) . ok () ? ; let f = | h : & str , n : & str | -> (Option < uid_t > , Option < uid_t >) { if h . starts_with (n) { let mut ids = h . split_whitespace () ; let real = ids . nth (1) . unwrap_or ("0") . parse () . ok () ; let effective = ids . next () . unwrap_or ("0") . parse () . ok () ; (real , effective) } else { (None , None) } } ; let mut uid = None ; let mut effective_uid = None ; let mut gid = None ; let mut effective_gid = None ; for line in status_data . lines () { if let (Some (real) , Some (effective)) = f (line , "Uid:") { debug_assert ! (uid . is_none () && effective_uid . is_none ()) ; uid = Some (real) ; effective_uid = Some (effective) ; } else if let (Some (real) , Some (effective)) = f (line , "Gid:") { debug_assert ! (gid . is_none () && effective_gid . is_none ()) ; gid = Some (real) ; effective_gid = Some (effective) ; } else { continue ; } if uid . is_some () && gid . is_some () { break ; } } match (uid , effective_uid , gid , effective_gid) { (Some (uid) , Some (effective_uid) , Some (gid) , Some (effective_gid)) => { Some (((uid , effective_uid) , (gid , effective_gid))) } _ => None , } }
};
}

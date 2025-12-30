// Generated macro for create_helper (function)
macro_rules! Depcrate_utilcreate_helper {
() => {
// Module: crate::util
// Provides: {"create_helper"}
// Dependencies: {}
pub fn create_helper < R > (base : & Path , prefix : & OsStr , suffix : & OsStr , random_len : usize , mut f : impl FnMut (PathBuf) -> io :: Result < R > ,) -> io :: Result < R > { let mut base = base ; let base_path_storage ; if ! base . is_absolute () { let cur_dir = std :: env :: current_dir () ? ; base_path_storage = cur_dir . join (base) ; base = & base_path_storage ; } let num_retries = if random_len != 0 { crate :: NUM_RETRIES } else { 1 } ; let mut rng = fastrand :: Rng :: new () ; for i in 0 .. num_retries { # [cfg (all (feature = "getrandom" , any (windows , unix , target_os = "redox" , target_os = "wasi")))] if i == 3 { if let Ok (seed) = getrandom :: u64 () { rng . seed (seed) ; } } let _ = i ; let path = base . join (tmpname (& mut rng , prefix , suffix , random_len)) ; return match f (path) { Err (ref e) if e . kind () == io :: ErrorKind :: AlreadyExists && num_retries > 1 => continue , Err (ref e) if e . kind () == io :: ErrorKind :: AddrInUse && num_retries > 1 => continue , res => res , } ; } Err (io :: Error :: new (io :: ErrorKind :: AlreadyExists , "too many temporary files exist" ,)) . with_err_path (| | base) }
};
}

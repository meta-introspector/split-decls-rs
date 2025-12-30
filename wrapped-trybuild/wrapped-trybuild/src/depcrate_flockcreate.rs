// Generated macro for create (function)
macro_rules! Depcrate_flockcreate {
() => {
// Module: crate::flock
// Provides: {"create"}
// Dependencies: {}
fn create (path : & Path) -> Option < File > { loop { match OpenOptions :: new () . write (true) . create_new (true) . open (path) { Ok (lockfile) => return Some (lockfile) , Err (io_error) => match io_error . kind () { io :: ErrorKind :: AlreadyExists => { } _ => return None , } , } let metadata = match fs :: metadata (path) { Ok (metadata) => metadata , Err (io_error) => match io_error . kind () { io :: ErrorKind :: NotFound => continue , _ => return None , } , } ; let Ok (modified) = metadata . modified () else { return None ; } ; let now = SystemTime :: now () ; let considered_stale = now - Duration :: from_millis (1500) ; let considered_future = now + Duration :: from_millis (1500) ; if modified < considered_stale || considered_future < modified { return File :: create (path) . ok () ; } thread :: sleep (Duration :: from_millis (500)) ; } }
};
}

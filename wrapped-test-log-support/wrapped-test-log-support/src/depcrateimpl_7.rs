// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl Test { pub fn start () -> Self { Self :: with_filters (& [("" , LevelFilter :: Trace)]) } pub fn with_filters < 'a > (filters : impl IntoIterator < Item = & 'a (& 'static str , LevelFilter) > ,) -> Self { let me = Arc :: new (State { last_log : Mutex :: new (None) , }) ; let state = me . clone () ; let mut max = LevelFilter :: Off ; let filters = filters . into_iter () . cloned () . inspect (| (_ , f) | { if f > & max { max = * f ; } }) . collect () ; let logger = Logger { filters , state : me } ; log :: set_boxed_logger (Box :: new (logger)) . unwrap () ; log :: set_max_level (max) ; Test { state } } # [track_caller] pub fn assert_logged (& self , expected : & str) { let last = match self . state . last_log . lock () . unwrap () . take () { Some (last) => last , _ => panic ! ("test failed: expected \"{}\", but nothing was logged" , expected) , } ; assert_eq ! (last . as_str () . trim () , expected) ; } # [track_caller] pub fn assert_not_logged (& self) { if let Some (last) = self . state . last_log . lock () . unwrap () . take () { panic ! ("test failed: nothing to be logged, but \"{}\" was logged" , last) ; } } }
};
}

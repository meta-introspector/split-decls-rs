// Generated macro for get (function)
macro_rules! Depcrateget {
() => {
// Module: crate
// Provides: {"get"}
// Dependencies: {}
pub fn get () -> impl Filter < Extract = impl Reply , Error = Rejection > + Clone { warp :: get () . and (path ! (String)) . and (filters :: ext :: get :: < State > ()) . map (| path : String , state : State | { let state = state . db . read () . unwrap () ; if let Some (value) = state . get (& path) . cloned () { Response :: new (value) } else { Response :: builder () . status (StatusCode :: NOT_FOUND) . body (Bytes :: new ()) . unwrap () } }) }
};
}

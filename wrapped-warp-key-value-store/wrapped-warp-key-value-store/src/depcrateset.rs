// Generated macro for set (function)
macro_rules! Depcrateset {
() => {
// Module: crate
// Provides: {"set"}
// Dependencies: {}
pub fn set () -> impl Filter < Extract = impl Reply , Error = Rejection > + Clone { warp :: post () . and (path ! (String)) . and (filters :: ext :: get :: < State > ()) . and (filters :: body :: bytes ()) . map (| path : String , state : State , value : Bytes | { let mut state = state . db . write () . unwrap () ; state . insert (path , value) ; Response :: new (Bytes :: new ()) }) }
};
}

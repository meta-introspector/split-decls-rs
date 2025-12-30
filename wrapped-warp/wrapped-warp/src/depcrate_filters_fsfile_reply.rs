// Generated macro for file_reply (function)
macro_rules! Depcrate_filters_fsfile_reply {
() => {
// Module: crate::filters::fs
// Provides: {"file_reply"}
// Dependencies: {}
fn file_reply (path : ArcPath , conditionals : Conditionals ,) -> impl Future < Output = Result < File , Rejection > > + Send { TkFile :: open (path . clone ()) . then (move | res | match res { Ok (f) => Either :: Left (file_conditional (f , path , conditionals)) , Err (err) => { let rej = match err . kind () { io :: ErrorKind :: NotFound => { tracing :: debug ! ("file not found: {:?}" , path . as_ref () . display ()) ; reject :: not_found () } io :: ErrorKind :: PermissionDenied => { tracing :: warn ! ("file permission denied: {:?}" , path . as_ref () . display ()) ; reject :: known (FilePermissionError { _p : () }) } _ => { tracing :: error ! ("file open error (path={:?}): {} " , path . as_ref () . display () , err) ; reject :: known (FileOpenError { _p : () }) } } ; Either :: Right (future :: err (rej)) } }) }
};
}

// Generated macro for file_metadata (function)
macro_rules! Depcrate_filters_fsfile_metadata {
() => {
// Module: crate::filters::fs
// Provides: {"file_metadata"}
// Dependencies: {}
async fn file_metadata (f : TkFile) -> Result < (TkFile , Metadata) , Rejection > { match f . metadata () . await { Ok (meta) => Ok ((f , meta)) , Err (err) => { tracing :: debug ! ("file metadata error: {}" , err) ; Err (reject :: not_found ()) } } }
};
}

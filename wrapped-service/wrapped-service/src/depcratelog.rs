// Generated macro for log (function)
macro_rules! Depcratelog {
() => {
// Module: crate
// Provides: {"log"}
// Dependencies: {}
fn log (message : & str) { unsafe { let file = CreateFileW (LOG_FILE , FILE_APPEND_DATA , 0 , std :: ptr :: null () , OPEN_ALWAYS , FILE_ATTRIBUTE_NORMAL , std :: ptr :: null_mut () ,) ; WriteFile (file , message . as_ptr () , message . len () . try_into () . unwrap () , & mut 0 , std :: ptr :: null_mut () ,) ; CloseHandle (file) ; } }
};
}

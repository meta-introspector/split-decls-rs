// Generated macro for TempFile (struct)
macro_rules! DepcrateTempFile {
() => {
// Module: crate
// Provides: {"TempFile"}
// Dependencies: {}
# [doc = " The path of an existing writable file in a system temporary directory."] # [doc = ""] # [doc = " Deletes the file on drop.  Ignores errors deleting the file."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " use temp_file::TempFile;"] # [doc = " let t = TempFile::new()"] # [doc = "   .unwrap()"] # [doc = "   .with_contents(b\"abc\")"] # [doc = "   .unwrap();"] # [doc = " // Prints \"/tmp/1a9b0\"."] # [doc = " println!(\"{:?}\", t.path());"] # [doc = " assert_eq!("] # [doc = "   \"abc\","] # [doc = "   std::fs::read_to_string(t.path()).unwrap(),"] # [doc = " );"] # [doc = " // Prints \"/tmp/1a9b1\"."] # [doc = " println!(\"{:?}\", TempFile::new().unwrap().path());"] # [doc = " ```"] # [derive (Clone , PartialOrd , Ord , PartialEq , Eq , Hash , Debug)] pub struct TempFile { path_buf : PathBuf , delete_on_drop : bool , panic_on_delete_err : bool , }
};
}

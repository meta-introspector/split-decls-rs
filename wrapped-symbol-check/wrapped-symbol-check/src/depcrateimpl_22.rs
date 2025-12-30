// Generated macro for impl_22 (impl)
macro_rules! Depcrateimpl_22 {
() => {
// Module: crate
// Provides: {"impl_22"}
// Dependencies: {}
impl BinFile { fn from_path (path : & Path) -> Self { Self { path : path . to_owned () , data : fs :: read (path) . expect ("reading file failed") , } } fn as_archive_file (& self) -> ObjResult < ArchiveFile < '_ > > { ArchiveFile :: parse (self . data . as_slice ()) } fn as_obj_file (& self) -> ObjResult < ObjFile < '_ > > { ObjFile :: parse (self . data . as_slice ()) } # [doc = " For a given archive, do something with each object file. For an object file, do"] # [doc = " something once."] fn for_each_object (& self , mut f : impl FnMut (ObjFile , & str)) { let as_archive = self . as_archive_file () ; if let Ok (archive) = as_archive { for member in archive . members () { let member = member . expect ("failed to access member") ; let obj_data = member . data (self . data . as_slice ()) . expect ("failed to access object") ; let obj = ObjFile :: parse (obj_data) . expect ("failed to parse object") ; f (obj , & String :: from_utf8_lossy (member . name ())) ; } return ; } let as_obj = self . as_obj_file () ; if let Ok (obj) = as_obj { f (obj , & self . path . to_string_lossy ()) ; return ; } panic ! ("failed to parse as either archive or object file: {:?}, {:?}" , as_archive . unwrap_err () , as_obj . unwrap_err () ,) ; } # [doc = " D something with each symbol in an archive or object file."] fn for_each_symbol (& self , mut f : impl FnMut (Symbol , & ObjFile , & str)) { self . for_each_object (| obj , obj_path | { obj . symbols () . for_each (| sym | f (sym , & obj , obj_path)) ; }) ; } }
};
}

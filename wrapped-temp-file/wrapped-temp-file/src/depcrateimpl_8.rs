// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl TempFileBuilder { # [allow (clippy :: new_without_default)] # [must_use] pub fn new () -> Self { Self { dir_path : None , prefix : None , suffix : None , } } # [must_use] pub fn in_dir (mut self , p : impl AsRef < Path >) -> Self { self . dir_path = Some (p . as_ref () . to_path_buf ()) ; self } # [must_use] pub fn prefix (mut self , s : impl AsRef < str >) -> Self { self . prefix = Some (s . as_ref () . to_string ()) ; self } # [must_use] pub fn suffix (mut self , s : impl AsRef < str >) -> Self { self . suffix = Some (s . as_ref () . to_string ()) ; self } # [doc = " Creates the temp file."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` when it fails to create the file."] pub fn build (self) -> Result < TempFile , std :: io :: Error > { TempFile :: internal_new (self . dir_path . as_deref () , self . prefix . as_ref () . map (AsRef :: as_ref) , self . suffix . as_ref () . map (AsRef :: as_ref) ,) } }
};
}

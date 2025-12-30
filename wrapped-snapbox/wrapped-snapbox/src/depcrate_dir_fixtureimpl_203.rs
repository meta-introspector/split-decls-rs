// Generated macro for impl_203 (impl)
macro_rules! Depcrate_dir_fixtureimpl_203 {
() => {
// Module: crate::dir::fixture
// Provides: {"impl_203"}
// Dependencies: {}
impl < P , S > DirFixture for & [(P , S)] where P : AsRef < std :: path :: Path > , P : std :: fmt :: Debug , S : AsRef < [u8] > , S : std :: fmt :: Debug , { fn write_to_path (& self , root : & std :: path :: Path) -> Result < () , crate :: assert :: Error > { let root = super :: ops :: canonicalize (root) . map_err (| e | format ! ("Failed to canonicalize {}: {}" , root . display () , e)) ? ; for (path , content) in self . iter () { let rel_path = path . as_ref () ; let path = root . join (rel_path) ; let path = super :: ops :: normalize_path (& path) ; if ! path . starts_with (& root) { return Err (crate :: assert :: Error :: new (format ! ("Fixture {} is for outside of the target root" , rel_path . display () ,))) ; } let content = content . as_ref () ; if let Some (dir) = path . parent () { std :: fs :: create_dir_all (dir) . map_err (| e | { format ! ("Failed to create fixture directory {}: {}" , dir . display () , e) }) ? ; } std :: fs :: write (& path , content) . map_err (| e | format ! ("Failed to write fixture {}: {}" , path . display () , e)) ? ; } Ok (()) } }
};
}

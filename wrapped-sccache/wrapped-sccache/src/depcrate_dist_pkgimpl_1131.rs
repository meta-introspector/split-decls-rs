// Generated macro for impl_1131 (impl)
macro_rules! Depcrate_dist_pkgimpl_1131 {
() => {
// Module: crate::dist::pkg
// Provides: {"impl_1131"}
// Dependencies: {}
impl SimplifyPath < '_ > { pub fn simplify (& mut self , path : & Path) -> Result < PathBuf > { let mut final_path = PathBuf :: new () ; for component in path . components () { match component { c @ Component :: RootDir | c @ Component :: Prefix (_) | c @ Component :: Normal (_) => { final_path . push (c) ; if self . resolved_symlinks . is_some () && final_path . is_symlink () { let parent = final_path . parent () . expect ("symlinks have parents") ; let link_target = final_path . read_link () ? ; let new_final_path = self . simplify (& parent . join (& link_target)) ? ; let old_final_path = std :: mem :: replace (& mut final_path , new_final_path . clone ()) ; self . resolved_symlinks . as_mut () . unwrap () . insert (old_final_path , new_final_path) ; } } Component :: ParentDir => { if final_path . is_symlink () { bail ! ("Cannot handle symlinks in parent paths") } final_path . pop () ; } Component :: CurDir => continue , } } Ok (final_path) } }
};
}

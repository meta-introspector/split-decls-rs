// Generated macro for impl_107 (impl)
macro_rules! Depcrate_ci_rsimpl_107 {
() => {
// Module: crate::ci::rs
// Provides: {"impl_107"}
// Dependencies: {}
impl Rs { pub fn run (mut self) -> Result < () > { let image = self . build () ? ; let arch = self . cargo_build . artifact . arch ; let small = self . cargo_build . artifact . profile () == "release" ; match self . action { Action :: Build => Ok (()) , Action :: Firecracker (firecracker) => firecracker . run (& image , self . smp) , Action :: Qemu (qemu) => qemu . run (& image , self . smp , arch , small) , Action :: Uhyve (uhyve) => uhyve . run (& image , self . smp) , } } pub fn build (& mut self) -> Result < PathBuf > { if super :: in_ci () { eprintln ! ("::group::cargo build") ; } if self . smp > 1 { self . cargo_build . features . push ("hermit/smp" . to_string ()) ; } let mut cargo = crate :: cargo () ; if self . package . contains ("rftrace") { cargo . env ("RUSTFLAGS" , "-Zinstrument-mcount -Cpasses=ee-instrument<post-inline>" ,) ; } ; cargo . current_dir (super :: parent_root ()) . arg ("build") . args (self . cargo_build . artifact . arch . ci_cargo_args ()) . args (self . cargo_build . cargo_build_args ()) . args (["--package" , self . package . as_str ()]) ; eprintln ! ("$ {cargo:?}") ; let status = cargo . status () ? ; assert ! (status . success ()) ; if super :: in_ci () { eprintln ! ("::endgroup::") ; } Ok (self . cargo_build . artifact . ci_image (& self . package)) } }
};
}

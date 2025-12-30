// Generated macro for impl_427 (impl)
macro_rules! Depcrate_compiler_cimpl_427 {
() => {
// Module: crate::compiler::c
// Provides: {"impl_427"}
// Dependencies: {}
impl < T : CommandCreatorSync , I : CCompilerImpl > Compiler < T > for CCompiler < I > { fn kind (& self) -> CompilerKind { CompilerKind :: C (self . compiler . kind ()) } # [cfg (feature = "dist-client")] fn get_toolchain_packager (& self) -> Box < dyn pkg :: ToolchainPackager > { Box :: new (CToolchainPackager { executable : self . executable . clone () , kind : self . compiler . kind () , }) } fn parse_arguments (& self , arguments : & [OsString] , cwd : & Path , env_vars : & [(OsString , OsString)] ,) -> CompilerArguments < Box < dyn CompilerHasher < T > + 'static > > { match self . compiler . parse_arguments (arguments , cwd , env_vars) { CompilerArguments :: Ok (mut args) => { for (k , v) in env_vars . iter () { if k . as_os_str () == OsStr :: new ("SCCACHE_EXTRAFILES") { args . extra_hash_files . extend (std :: env :: split_paths (& v)) ; } } if args . language == Language :: Hip { args . extra_hash_files . extend (Self :: search_hip_device_libs (& args , env_vars)) ; } CompilerArguments :: Ok (Box :: new (CCompilerHasher { parsed_args : args , executable : self . executable . clone () , executable_digest : self . executable_digest . clone () , compiler : self . compiler . clone () , })) } CompilerArguments :: CannotCache (why , extra_info) => { CompilerArguments :: CannotCache (why , extra_info) } CompilerArguments :: NotCompilation => CompilerArguments :: NotCompilation , } } fn box_clone (& self) -> Box < dyn Compiler < T > > { Box :: new ((* self) . clone ()) } }
};
}

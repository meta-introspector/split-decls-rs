// Generated macro for Compilation (trait)
macro_rules! Depcrate_compiler_compilerCompilation {
() => {
// Module: crate::compiler::compiler
// Provides: {"Compilation"}
// Dependencies: {}
# [doc = " An interface to a compiler for actually invoking compilation."] pub trait Compilation < T > : Send where T : CommandCreatorSync , { # [doc = " Given information about a compiler command, generate a command that can"] # [doc = " execute the compiler."] fn generate_compile_commands (& self , path_transformer : & mut dist :: PathTransformer , rewrite_includes_only : bool ,) -> Result < (Box < dyn CompileCommand < T > > , Option < dist :: CompileCommand > , Cacheable ,) > ; # [doc = " Create a function that will create the inputs used to perform a distributed compilation"] # [cfg (feature = "dist-client")] fn into_dist_packagers (self : Box < Self > , _path_transformer : dist :: PathTransformer ,) -> Result < DistPackagers > ; fn is_locally_preprocessed (& self) -> bool { true } # [doc = " Returns an iterator over the results of this compilation."] # [doc = ""] # [doc = " Each item is a descriptive (and unique) name of the output paired with"] # [doc = " the path where it'll show up."] fn outputs < 'a > (& 'a self) -> Box < dyn Iterator < Item = FileObjectSource > + 'a > ; }
};
}

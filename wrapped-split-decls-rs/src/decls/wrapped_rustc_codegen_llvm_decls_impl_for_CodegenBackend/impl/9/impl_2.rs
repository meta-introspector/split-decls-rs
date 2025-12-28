use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CodegenBackend for LlvmCodegenBackend { fn locale_resource (& self) -> & 'static str { crate :: DEFAULT_LOCALE_RESOURCE } fn name (& self) -> & 'static str { "llvm" } fn init (& self , sess : & Session) { llvm_util :: init (sess) ; } fn provide (& self , providers : & mut Providers) { providers . global_backend_features = | tcx , () | llvm_util :: global_llvm_features (tcx . sess , false) ; } fn print (& self , req : & PrintRequest , out : & mut String , sess : & Session) { use std :: fmt :: Write ; match req . kind { PrintKind :: RelocationModels => { writeln ! (out , "Available relocation models:") . unwrap () ; for name in RelocModel :: ALL . iter () . map (RelocModel :: desc) . chain (["default"]) { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: CodeModels => { writeln ! (out , "Available code models:") . unwrap () ; for name in & ["tiny" , "small" , "kernel" , "medium" , "large"] { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: TlsModels => { writeln ! (out , "Available TLS models:") . unwrap () ; for name in TlsModel :: ALL . iter () . map (TlsModel :: desc) { writeln ! (out , "    {name}") . unwrap () ; } writeln ! (out) . unwrap () ; } PrintKind :: StackProtectorStrategies => { writeln ! (out , r#"Available stack protector strategies:
    all
        Generate stack canaries in all functions.

    strong
        Generate stack canaries in a function if it either:
        - has a local variable of `[T; N]` type, regardless of `T` and `N`
        - takes the address of a local variable.

          (Note that a local variable being borrowed is not equivalent to its
          address being taken: e.g. some borrows may be removed by optimization,
          while by-value argument passing may be implemented with reference to a
          local stack variable in the ABI.)

    basic
        Generate stack canaries in functions with local variables of `[T; N]`
        type, where `T` is byte-sized and `N` >= 8.

    none
        Do not generate stack canaries.
"#) . unwrap () ; } _other => llvm_util :: print (req , out , sess) , } } fn print_passes (& self) { llvm_util :: print_passes () ; } fn print_version (& self) { llvm_util :: print_version () ; } fn target_config (& self , sess : & Session) -> TargetConfig { target_config (sess) } fn codegen_crate < 'tcx > (& self , tcx : TyCtxt < 'tcx >) -> Box < dyn Any > { Box :: new (rustc_codegen_ssa :: base :: codegen_crate (LlvmCodegenBackend (()) , tcx , crate :: llvm_util :: target_cpu (tcx . sess) . to_string () ,)) } fn join_codegen (& self , ongoing_codegen : Box < dyn Any > , sess : & Session , outputs : & OutputFilenames ,) -> (CodegenResults , FxIndexMap < WorkProductId , WorkProduct >) { let (codegen_results , work_products) = ongoing_codegen . downcast :: < rustc_codegen_ssa :: back :: write :: OngoingCodegen < LlvmCodegenBackend > > () . expect ("Expected LlvmCodegenBackend's OngoingCodegen, found Box<Any>") . join (sess) ; if sess . opts . unstable_opts . llvm_time_trace { sess . time ("llvm_dump_timing_file" , | | { let file_name = outputs . with_extension ("llvm_timings.json") ; llvm_util :: time_trace_profiler_finish (& file_name) ; }) ; } (codegen_results , work_products) } fn link (& self , sess : & Session , codegen_results : CodegenResults , metadata : EncodedMetadata , outputs : & OutputFilenames ,) { use crate :: back :: archive :: LlvmArchiveBuilderBuilder ; use rustc_codegen_ssa :: back :: link :: link_binary ; link_binary (sess , & LlvmArchiveBuilderBuilder , codegen_results , metadata , outputs , self . name () ,) ; } }
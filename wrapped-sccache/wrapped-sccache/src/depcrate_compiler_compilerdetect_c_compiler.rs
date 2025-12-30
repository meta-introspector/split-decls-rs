// Generated macro for detect_c_compiler (function)
macro_rules! Depcrate_compiler_compilerdetect_c_compiler {
() => {
// Module: crate::compiler::compiler
// Provides: {"detect_c_compiler"}
// Dependencies: {}
async fn detect_c_compiler < T , P > (creator : T , executable : P , arguments : & [OsString] , env : Vec < (OsString , OsString) > , pool : tokio :: runtime :: Handle ,) -> Result < Box < dyn Compiler < T > > > where T : CommandCreatorSync , P : AsRef < Path > , { trace ! ("detect_c_compiler") ; let test = b"
#if defined(__NVCC__) && defined(__NVCOMPILER)
compiler_id=nvcc-nvhpc
compiler_version=__CUDACC_VER_MAJOR__.__CUDACC_VER_MINOR__.__CUDACC_VER_BUILD__
#elif defined(__NVCC__) && defined(_MSC_VER)
compiler_id=nvcc-msvc
compiler_version=__CUDACC_VER_MAJOR__.__CUDACC_VER_MINOR__.__CUDACC_VER_BUILD__
#elif defined(__NVCC__)
compiler_id=nvcc
compiler_version=__CUDACC_VER_MAJOR__.__CUDACC_VER_MINOR__.__CUDACC_VER_BUILD__
#elif defined(_MSC_VER) && !defined(__clang__)
compiler_id=msvc
#elif defined(_MSC_VER) && defined(_MT)
compiler_id=msvc-clang
#elif defined(__NVCOMPILER)
compiler_id=nvhpc
compiler_version=__NVCOMPILER_MAJOR__.__NVCOMPILER_MINOR__.__NVCOMPILER_PATCHLEVEL__
#elif defined(__clang__) && defined(__cplusplus) && defined(__apple_build_version__)
compiler_id=apple-clang++
#elif defined(__clang__) && defined(__cplusplus)
compiler_id=clang++
#elif defined(__clang__) && defined(__apple_build_version__)
compiler_id=apple-clang
#elif defined(__clang__)
compiler_id=clang
#elif defined(__GNUC__) && defined(__cplusplus)
compiler_id=g++
#elif defined(__GNUC__)
compiler_id=gcc
#elif defined(__DCC__)
compiler_id=diab
#elif defined(__CTC__)
compiler_id=tasking_vx
#else
compiler_id=unknown
#endif
compiler_version=__VERSION__
" . to_vec () ; let (tempdir , src) = write_temp_file (& pool , "testfile.c" . as_ref () , test) . await ? ; let executable = executable . as_ref () ; let mut cmd = creator . clone () . new_command_sync (executable) ; cmd . stdout (Stdio :: piped ()) . stderr (Stdio :: piped ()) . envs (env . iter () . map (| s | (& s . 0 , & s . 1))) ; for arg in ArgsIter :: new (arguments . iter () . cloned () , & ARGS [..]) { let arg = arg . unwrap_or_else (| _ | Argument :: Raw (OsString :: from (""))) ; if let Some (Detect_PassThrough (_)) = arg . get_data () { let required_arg = arg . normalize (NormalizedDisposition :: Concatenated) ; cmd . args (& Vec :: from_iter (required_arg . iter_os_strings ())) ; } } cmd . arg ("-E") . arg (src) ; trace ! ("compiler {:?}" , cmd) ; let child = cmd . spawn () . await ? ; let output = child . wait_with_output () . await . context ("failed to read child output") ? ; drop (tempdir) ; let stdout = match str :: from_utf8 (& output . stdout) { Ok (s) => s , Err (_) => bail ! ("Failed to parse output") , } ; let mut lines = stdout . lines () . filter_map (| line | { let line = line . trim () ; if line . starts_with ("compiler_id=") { Some (line . strip_prefix ("compiler_id=") . unwrap ()) } else if line . starts_with ("compiler_version=") { Some (line . strip_prefix ("compiler_version=") . unwrap ()) } else { None } }) ; if let Some (kind) = lines . next () { let executable = executable . to_owned () ; let version = lines . next () . filter (| & line | line != "__VERSION__") . map (str :: to_owned) ; match kind { "clang" | "clang++" | "apple-clang" | "apple-clang++" => { debug ! ("Found {}" , kind) ; return CCompiler :: new (Clang { clangplusplus : kind . ends_with ("++") , is_appleclang : kind . starts_with ("apple-") , version : version . clone () , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "diab" => { debug ! ("Found diab") ; return CCompiler :: new (Diab { version : version . clone () , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "gcc" | "g++" => { debug ! ("Found {}" , kind) ; return CCompiler :: new (Gcc { gplusplus : kind == "g++" , version : version . clone () , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "msvc" | "msvc-clang" => { let is_clang = kind == "msvc-clang" ; debug ! ("Found MSVC (is clang: {})" , is_clang) ; let prefix = msvc :: detect_showincludes_prefix (& creator , executable . as_ref () , is_clang , env , & pool ,) . await ? ; trace ! ("showIncludes prefix: '{}'" , prefix) ; return CCompiler :: new (Msvc { includes_prefix : prefix , is_clang , version : version . clone () , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "nvcc" | "nvcc-msvc" | "nvcc-nvhpc" => { let host_compiler = match kind { "nvcc-nvhpc" => NvccHostCompiler :: Nvhpc , "nvcc-msvc" => NvccHostCompiler :: Msvc , "nvcc" => NvccHostCompiler :: Gcc , & _ => NvccHostCompiler :: Gcc , } ; let host_compiler_version = lines . next () . filter (| & line | line != "__VERSION__") . map (str :: to_owned) ; return CCompiler :: new (Nvcc { host_compiler , version , host_compiler_version , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "nvhpc" => { debug ! ("Found NVHPC") ; return CCompiler :: new (Nvhpc { nvcplusplus : kind == "nvc++" , version : version . clone () , } , executable , & pool ,) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } "tasking_vx" => { debug ! ("Found Tasking VX") ; return CCompiler :: new (TaskingVX , executable , & pool) . await . map (| c | Box :: new (c) as Box < dyn Compiler < T > >) ; } _ => () , } } let stderr = String :: from_utf8_lossy (& output . stderr) ; debug ! ("nothing useful in detection output {:?}" , stdout) ; debug ! ("compiler status: {}" , output . status) ; debug ! ("compiler stderr:\n{}" , stderr) ; bail ! (stderr . into_owned ()) }
};
}

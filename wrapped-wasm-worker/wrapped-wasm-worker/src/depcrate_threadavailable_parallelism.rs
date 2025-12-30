// Generated macro for available_parallelism (function)
macro_rules! Depcrate_threadavailable_parallelism {
() => {
// Module: crate::thread
// Provides: {"available_parallelism"}
// Dependencies: {}
# [doc = " See [`std::thread::available_parallelism()`]."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Browsers might return lower values, a common case is to prevent"] # [doc = " fingerprinting."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error if called from a worklet or any other"] # [doc = " unsupported thread type."] # [allow (clippy :: missing_panics_doc)] pub fn available_parallelism () -> io :: Result < NonZeroUsize > { let value = Global :: with (| global | match global { Global :: Window (window) => Ok (window . navigator () . hardware_concurrency ()) , Global :: Dedicated (worker) => Ok (worker . navigator () . hardware_concurrency ()) , Global :: Shared (worker) => Ok (worker . navigator () . hardware_concurrency ()) , Global :: Service (worker) | Global :: Worker (worker) => { Ok (worker . navigator () . hardware_concurrency ()) } Global :: Worklet => Err (Error :: new (ErrorKind :: Unsupported , "operation not supported in worklets" ,)) , Global :: Unknown => Err (Error :: new (ErrorKind :: Unsupported , "encountered unsupported thread type" ,)) , }) ? ; # [allow (clippy :: as_conversions , clippy :: cast_possible_truncation , clippy :: cast_sign_loss)] let value = value as usize ; let value = NonZeroUsize :: new (value) . expect ("`Navigator.hardwareConcurrency` returned an unexpected value of `0`") ; Ok (value) }
};
}

macro_rules! CompilerError {
    () => {
        # [doc = " An error type used to represent an error that has already been reported by the compiler."] # [derive (Clone , Copy , PartialEq , Eq)] pub enum CompilerError < T > { # [doc = " Compilation failed, either due to normal errors or ICE."] Failed , # [doc = " Compilation was interrupted."] Interrupted (T) , # [doc = " Compilation skipped. This happens when users invoke rustc to retrieve information such as"] # [doc = " --version."] Skipped , }
    };
}

CompilerError!()
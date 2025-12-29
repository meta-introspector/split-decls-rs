// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_140",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation", "SecureExecution", "DefaultSecurity", "SecurityError"],
uses: ["AstOperation", "SecureExecution", "FnOnce", "DefaultSecurity", "Result", "Ok", "SecurityError"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AstOperation!();
        SecureExecution!();
        DefaultSecurity!();
        SecurityError!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl SecureExecution for DefaultSecurity { fn check_permission (& self , _operation : & AstOperation) -> bool { true } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
    };
}

impl_140!();
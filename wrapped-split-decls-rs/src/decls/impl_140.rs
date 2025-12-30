// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_140",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["SecureExecution", "SecurityError", "AstOperation", "DefaultSecurity"],
uses: ["SecureExecution", "SecurityError", "AstOperation", "Result", "Ok", "FnOnce", "DefaultSecurity"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        SecureExecution!();
        SecurityError!();
        AstOperation!();
        DefaultSecurity!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl SecureExecution for DefaultSecurity { fn check_permission (& self , _operation : & AstOperation) -> bool { true } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
    };
}

impl_140!();
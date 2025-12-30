// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_143",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["StrictSecurity", "AstOperation", "SecurityError", "SecureExecution"],
uses: ["StrictSecurity", "AstOperation", "Result", "SecurityError", "FnOnce", "SecureExecution", "Ok"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        StrictSecurity!();
        AstOperation!();
        SecurityError!();
        SecureExecution!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl SecureExecution for StrictSecurity { fn check_permission (& self , operation : & AstOperation) -> bool { self . allowed_operations . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
    };
}

impl_143!();
// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "impl_143",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["SecurityError", "SecureExecution", "AstOperation", "StrictSecurity"],
uses: ["Result", "SecurityError", "SecureExecution", "AstOperation", "FnOnce", "StrictSecurity", "Ok"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SecurityError!();
        SecureExecution!();
        AstOperation!();
        StrictSecurity!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl SecureExecution for StrictSecurity { fn check_permission (& self , operation : & AstOperation) -> bool { self . allowed_operations . iter () . any (| op | { std :: mem :: discriminant (op) == std :: mem :: discriminant (operation) }) } fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T { Ok (f ()) } }
    };
}

impl_143!();
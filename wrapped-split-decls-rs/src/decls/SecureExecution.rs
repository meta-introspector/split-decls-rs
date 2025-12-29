// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "SecureExecution",
decl_type: "function",
source_file: "./src/syn2macro.rs",
source_crate: ".",
deps: ["AstOperation", "SecurityError"],
uses: ["Result", "AST", "AstOperation", "SecurityError", "SecureExecution", "FnOnce", "Security", "ACL"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        AstOperation!();
        SecurityError!();
    };
}

macro_rules! SecureExecution {
    () => {
        deps!();
        # [doc = " Security and ACL trait for AST operations"] pub trait SecureExecution { fn check_permission (& self , operation : & AstOperation) -> bool ; fn sandbox_execute < F , T > (& self , f : F) -> Result < T , SecurityError > where F : FnOnce () -> T ; }
    };
}

SecureExecution!();
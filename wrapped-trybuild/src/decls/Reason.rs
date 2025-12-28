macro_rules! Reason {
    () => {
        # [derive (Deserialize)] enum Reason { # [serde (rename = "compiler-message")] CompilerMessage , }
    };
}

Reason!();
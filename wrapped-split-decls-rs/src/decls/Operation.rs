// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Operation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["ShellCommandOperation", "FunctionCallOperation", "LoopOperation", "SwitchOperation", "SequenceOperation"],
uses: ["FunctionCall", "Serialize", "Value", "Switch", "ShellCommandOperation", "Debug", "Sequence", "FunctionCallOperation", "LoopOperation", "Operation", "Deserialize", "Loop", "SwitchOperation", "SequenceOperation", "Clone", "Unknown", "Shell"],
fields: [],
generated_at: "2025-12-29 17:10:19 UTC"
});

macro_rules! deps {
    () => {
        ShellCommandOperation!();
        FunctionCallOperation!();
        LoopOperation!();
        SwitchOperation!();
        SequenceOperation!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] # [serde (untagged)] pub enum Operation { FunctionCall (FunctionCallOperation) , Loop (LoopOperation) , Sequence (SequenceOperation) , Switch (SwitchOperation) , Shell (ShellCommandOperation) , # [serde (untagged)] Unknown (toml :: Value) , }
    };
}

Operation!();
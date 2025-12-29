// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "Operation",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["SequenceOperation", "LoopOperation", "ShellCommandOperation", "SwitchOperation", "FunctionCallOperation"],
uses: ["SequenceOperation", "Shell", "Serialize", "Clone", "LoopOperation", "Loop", "Value", "ShellCommandOperation", "Sequence", "FunctionCall", "Switch", "Debug", "Operation", "Unknown", "SwitchOperation", "Deserialize", "FunctionCallOperation"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        SequenceOperation!();
        LoopOperation!();
        ShellCommandOperation!();
        SwitchOperation!();
        FunctionCallOperation!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] # [serde (untagged)] pub enum Operation { FunctionCall (FunctionCallOperation) , Loop (LoopOperation) , Sequence (SequenceOperation) , Switch (SwitchOperation) , Shell (ShellCommandOperation) , # [serde (untagged)] Unknown (toml :: Value) , }
    };
}

Operation!();
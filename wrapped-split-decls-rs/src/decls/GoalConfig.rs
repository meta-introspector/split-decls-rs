// STRUCTURED DECLARATION METADATA
decl_metadata!({
name: "GoalConfig",
decl_type: "function",
source_file: "./src/goal_parser.rs",
source_crate: ".",
deps: ["Workflow"],
uses: ["Option", "Serialize", "GoalConfig", "String", "Workflow", "Clone", "Debug", "Deserialize"],
fields: [],
generated_at: "2025-12-29 16:02:27 UTC"
});

macro_rules! deps {
    () => {
        Workflow!();
    };
}

macro_rules! GoalConfig {
    () => {
        deps!();
        # [derive (Debug , Deserialize , Serialize , Clone)] pub struct GoalConfig { # [serde (rename = "original-goal")] pub original_goal : Option < String > , pub workflow : Workflow , }
    };
}

GoalConfig!();
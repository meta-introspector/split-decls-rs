// Generated macro for extract_workflow_typos_version (function)
macro_rules! Depcrateextract_workflow_typos_version {
() => {
// Module: crate
// Provides: {"extract_workflow_typos_version"}
// Dependencies: {}
fn extract_workflow_typos_version (metadata : & Metadata) -> anyhow :: Result < Version > { let ws_root = metadata . workspace_root . as_path () . as_std_path () ; let workflow_path = ws_root . join (".github") . join ("workflows") . join ("main.yml") ; let file_content = std :: fs :: read_to_string (workflow_path) ? ; if let Some (line) = file_content . lines () . find (| line | line . contains (TYPOS_STEP_PREFIX)) && let Some (stripped) = line . strip_prefix (TYPOS_STEP_PREFIX) && let Ok (v) = Version :: parse (stripped) { Ok (v) } else { Err (anyhow :: anyhow ! ("Could not find typos version in workflow")) } }
};
}

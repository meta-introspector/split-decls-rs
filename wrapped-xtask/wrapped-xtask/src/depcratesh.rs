// Generated macro for sh (function)
macro_rules! Depcratesh {
() => {
// Module: crate
// Provides: {"sh"}
// Dependencies: {}
pub fn sh () -> Result < Shell > { let sh = Shell :: new () ? ; sh . change_dir (project_root ()) ; Ok (sh) }
};
}

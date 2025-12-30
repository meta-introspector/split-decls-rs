// Generated macro for ModResolver (struct)
macro_rules! Depcrate_modulesModResolver {
() => {
// Module: crate::modules
// Provides: {"ModResolver"}
// Dependencies: {}
# [doc = " Maps each module to the corresponding file."] pub (crate) struct ModResolver < 'ast , 'psess > { psess : & 'psess ParseSess , directory : Directory , file_map : FileModMap < 'ast > , recursive : bool , }
};
}

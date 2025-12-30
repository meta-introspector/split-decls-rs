// Generated macro for install (function)
macro_rules! Depcrate_targetsinstall {
() => {
// Module: crate::targets
// Provides: {"install"}
// Dependencies: {}
# [doc = " Make sure a fixed set of compilation targets is installed"] # [doc = ""] # [doc = " Returns the `added_targets`."] pub fn install () -> anyhow :: Result < Vec < String > > { let required_targets = ["thumbv6m-none-eabi" , "thumbv7m-none-eabi" , "thumbv7em-none-eabi" , "thumbv8m.base-none-eabi" , "riscv32i-unknown-none-elf" ,] . iter () . map (| item | item . to_string ()) . collect :: < HashSet < _ > > () ; let added_targets = required_targets . difference (& get_installed () ?) . cloned () . collect () ; println ! ("⏳ installing targets") ; let status = Command :: new ("rustup") . args (["target" , "add"]) . args (& required_targets) . status () ? ; if ! status . success () { panic ! ("Error installing targets (see output above)") ; } Ok (added_targets) }
};
}

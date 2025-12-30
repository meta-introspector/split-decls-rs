// Generated macro for macro_757 (macro)
macro_rules! Depcrate_specmacro_757 {
() => {
// Module: crate::spec
// Provides: {"macro_757"}
// Dependencies: {}
crate :: target_spec_enum ! { # [derive (Default)] pub enum SplitDebuginfo { # [doc = " Split debug-information is disabled, meaning that on supported platforms"] # [doc = " you can find all debug information in the executable itself. This is"] # [doc = " only supported for ELF effectively."] # [doc = ""] # [doc = " * Windows - not supported"] # [doc = " * macOS - don't run `dsymutil`"] # [doc = " * ELF - `.debug_*` sections"] # [default] Off = "off" , # [doc = " Split debug-information can be found in a \"packed\" location separate"] # [doc = " from the final artifact. This is supported on all platforms."] # [doc = ""] # [doc = " * Windows - `*.pdb`"] # [doc = " * macOS - `*.dSYM` (run `dsymutil`)"] # [doc = " * ELF - `*.dwp` (run `thorin`)"] Packed = "packed" , # [doc = " Split debug-information can be found in individual object files on the"] # [doc = " filesystem. The main executable may point to the object files."] # [doc = ""] # [doc = " * Windows - not supported"] # [doc = " * macOS - supported, scattered object files"] # [doc = " * ELF - supported, scattered `*.dwo` or `*.o` files (see `SplitDwarfKind`)"] Unpacked = "unpacked" , } parse_error_type = "split debuginfo" ; }
};
}

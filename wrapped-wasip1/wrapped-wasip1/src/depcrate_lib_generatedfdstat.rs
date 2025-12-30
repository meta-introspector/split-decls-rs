// Generated macro for Fdstat (struct)
macro_rules! Depcrate_lib_generatedFdstat {
() => {
// Module: crate::lib_generated
// Provides: {"Fdstat"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone , Debug)] pub struct Fdstat { # [doc = " File type."] pub fs_filetype : Filetype , # [doc = " File descriptor flags."] pub fs_flags : Fdflags , # [doc = " Rights that apply to this file descriptor."] pub fs_rights_base : Rights , # [doc = " Maximum set of rights that may be installed on new file descriptors that"] # [doc = " are created through this file descriptor, e.g., through `path_open`."] pub fs_rights_inheriting : Rights , }
};
}

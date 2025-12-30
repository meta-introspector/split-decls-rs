// Generated macro for fmap (function)
macro_rules! Depcrate_callfmap {
() => {
// Module: crate::call
// Provides: {"fmap"}
// Dependencies: {}
# [doc = " Map a file into memory, but with the ability to set the address to map into, either as a hint"] # [doc = " or as a requirement of the map."] # [doc = ""] # [doc = " # Errors"] # [doc = " `EACCES` - the file descriptor was not open for reading"] # [doc = " `EBADF` - if the file descriptor was invalid"] # [doc = " `ENODEV` - mmapping was not supported"] # [doc = " `EINVAL` - invalid combination of flags"] # [doc = " `EEXIST` - if [`MapFlags::MAP_FIXED`] was set, and the address specified was already in use."] # [doc = ""] pub unsafe fn fmap (fd : usize , map : & Map) -> Result < usize > { syscall3 (SYS_FMAP , fd , map as * const Map as usize , mem :: size_of :: < Map > () ,) }
};
}

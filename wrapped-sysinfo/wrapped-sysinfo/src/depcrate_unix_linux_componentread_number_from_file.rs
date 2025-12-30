// Generated macro for read_number_from_file (function)
macro_rules! Depcrate_unix_linux_componentread_number_from_file {
() => {
// Module: crate::unix::linux::component
// Provides: {"read_number_from_file"}
// Dependencies: {}
# [doc = " Designed at first for reading an `i32` or `u32` aka `c_long`"] # [doc = " from a `/sys/class/hwmon` sysfs file."] fn read_number_from_file < N > (file : & Path) -> Option < N > where N : std :: str :: FromStr , { let mut reader = [0u8 ; 32] ; let mut f = File :: open (file) . ok () ? ; let n = f . read (& mut reader) . ok () ? ; let number = & reader [.. n] ; let number = std :: str :: from_utf8 (number) . ok () ? ; let number = number . trim () ; if cfg ! (feature = "debug") { assert ! (! number . contains ('\n') && ! number . contains ('\0')) ; } number . parse () . ok () }
};
}

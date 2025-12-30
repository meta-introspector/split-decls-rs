// Generated macro for sim (module)
macro_rules! Depcratesim {
() => {
// Module: crate
// Provides: {"sim"}
// Dependencies: {}
mod sim { use core :: fmt ; extern "C" { fn putchar (ch : i32) ; pub fn _exit (code : i32) -> ! ; } pub struct Console ; impl fmt :: Write for Console { fn write_str (& mut self , s : & str) -> fmt :: Result { for & b in s . as_bytes () { unsafe { putchar (b as i32) } } Ok (()) } } }
};
}

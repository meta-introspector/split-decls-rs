macro_rules! Winsize {
    () => {
        # [doc = " `struct winsize` for use with [`tcgetwinsize`]."] # [doc = ""] # [doc = " [`tcgetwinsize`]: crate::termios::tcgetwinsize"] # [doc (alias = "winsize")] # [repr (C)] # [derive (Debug , Copy , Clone , Eq , Hash , PartialEq)] # [allow (missing_docs)] pub struct Winsize { # [doc = " The number of rows the terminal has."] pub ws_row : u16 , # [doc = " The number of columns the terminal has."] pub ws_col : u16 , pub ws_xpixel : u16 , pub ws_ypixel : u16 , }
    };
}

Winsize!();
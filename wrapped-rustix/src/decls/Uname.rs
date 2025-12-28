macro_rules! Uname {
    () => {
        # [doc = " `struct utsname`—Return type for [`uname`]."] # [doc (alias = "utsname")] pub struct Uname (backend :: system :: types :: RawUname) ;
    };
}

Uname!()
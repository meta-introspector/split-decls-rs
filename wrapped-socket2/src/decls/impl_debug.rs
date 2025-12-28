macro_rules! impl_debug {
    () => {
        # [doc = " Macro to implement `fmt::Debug` for a type, printing the constant names"] # [doc = " rather than a number."] # [doc = ""] # [doc = " Note this is used in the `sys` module and thus must be defined before"] # [doc = " defining the modules."] macro_rules ! impl_debug { ($ type : path , $ ($ (# [$ target : meta]) * $ libc : ident :: $ flag : ident) ,+ $ (,) *) => { impl std :: fmt :: Debug for $ type { fn fmt (& self , f : & mut std :: fmt :: Formatter <'_ >) -> std :: fmt :: Result { let string = match self . 0 { $ ($ (# [$ target]) * $ libc :: $ flag => stringify ! ($ flag) ,) + n => return write ! (f , "{n}") , } ; f . write_str (string) } } } ; }
    };
}

impl_debug!()
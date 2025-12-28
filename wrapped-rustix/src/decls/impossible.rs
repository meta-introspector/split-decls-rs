macro_rules! impossible {
    () => {
        # [doc = " Functions which don't seem possible to even call from Rust with current"] # [doc = " language features, even with `unsafe`."] pub mod impossible { not_implemented ! (vfork) ; not_implemented ! (sigreturn) ; not_implemented ! (setjmp) ; not_implemented ! (longjmp) ; not_implemented ! (sigsetjmp) ; not_implemented ! (siglongjmp) ; }
    };
}

impossible!();
macro_rules! KernelSigrestore {
    () => {
        # [doc = " `__sigrestore_t`"] # [doc = ""] # [doc = " This type differs from `libc::sigrestore_t`, but can be transmuted to it."] pub type KernelSigrestore = Option < unsafe extern "C" fn () > ;
    };
}

KernelSigrestore!()
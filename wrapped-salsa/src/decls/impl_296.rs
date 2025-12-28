macro_rules! deps {
    () => {
        ZalsaDatabase!();
        Zalsa!();
        ZalsaLocal!();
        RawDatabase!();
        HasStorage!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl < T : HasStorage > ZalsaDatabase for T { # [inline (always)] fn zalsa (& self) -> & Zalsa { & self . storage () . handle . zalsa_impl } fn zalsa_mut (& mut self) -> & mut Zalsa { self . storage_mut () . cancel_others () } # [inline (always)] fn zalsa_local (& self) -> & ZalsaLocal { & self . storage () . zalsa_local } # [inline (always)] fn fork_db (& self) -> RawDatabase < 'static > { Box :: leak (Box :: new (self . clone ())) . into () } }
    };
}

impl_296!();
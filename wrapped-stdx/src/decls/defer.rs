macro_rules! defer {
    () => {
        # [must_use] pub fn defer < F : FnOnce () > (f : F) -> impl Drop { struct D < F : FnOnce () > (Option < F >) ; impl < F : FnOnce () > Drop for D < F > { fn drop (& mut self) { if let Some (f) = self . 0 . take () { f () ; } } } D (Some (f)) }
    };
}

defer!()
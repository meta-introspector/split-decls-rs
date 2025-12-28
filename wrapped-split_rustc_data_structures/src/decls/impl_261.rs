macro_rules! deps {
    () => {
        DynSync!();
        DynSend!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }
    };
}

impl_261!();
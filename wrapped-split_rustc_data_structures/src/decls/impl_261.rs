macro_rules! deps {
    () => {
        DynSend!();
        DynSync!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        unsafe impl < T : DynSync + ? Sized + PointeeSized > DynSend for & T { }
    };
}

impl_261!()
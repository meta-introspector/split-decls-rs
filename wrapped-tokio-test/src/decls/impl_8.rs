macro_rules! deps {
    () => {
        Mock!();
        Action!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Mock { fn maybe_wakeup_reader (& mut self) { match self . inner . action () { Some (& mut Action :: Read (_)) | Some (& mut Action :: ReadError (_)) | None => { if let Some (waker) = self . inner . read_wait . take () { waker . wake () ; } } _ => { } } } }
    };
}

impl_8!();
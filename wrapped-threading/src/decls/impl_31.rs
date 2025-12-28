macro_rules! deps {
    () => {
        Pool!();
        Scope!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'scope , 'env > Scope < 'scope , 'env > { # [doc = " Submits the closure to run on the `Pool`."] # [doc = ""] # [doc = " The closure cannot outlive the `Scope` it's run in."] pub fn submit < F : FnOnce () + Send + 'scope > (& 'scope self , f : F) { unsafe { try_submit (& * self . pool . 0 , f) ; } } }
    };
}

impl_31!()
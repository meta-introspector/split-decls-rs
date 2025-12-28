macro_rules! CallOnDrop {
    () => {
        struct CallOnDrop < O , F : FnOnce () -> O > { f : ManuallyDrop < F > , }
    };
}

CallOnDrop!()
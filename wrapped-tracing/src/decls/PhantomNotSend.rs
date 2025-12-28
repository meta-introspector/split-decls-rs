macro_rules! PhantomNotSend {
    () => {
        # [allow (non_upper_case_globals)] const PhantomNotSend : PhantomNotSend = PhantomNotSend { ghost : PhantomData } ;
    };
}

PhantomNotSend!()
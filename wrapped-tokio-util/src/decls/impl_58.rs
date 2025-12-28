macro_rules! deps {
    () => {
        PollSendError!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < T : fmt :: Debug > std :: error :: Error for PollSendError < T > { }
    };
}

impl_58!();
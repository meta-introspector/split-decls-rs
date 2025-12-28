macro_rules! deps {
    () => {
        NoSubscriber!();
    };
}

macro_rules! NO_SUBSCRIBER {
    () => {
        deps!();
        static NO_SUBSCRIBER : NoSubscriber = NoSubscriber :: new () ;
    };
}

NO_SUBSCRIBER!();
macro_rules! deps {
    () => {
        Action!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        struct Inner { actions : VecDeque < Action > , waiting : Option < Instant > , sleep : Option < Pin < Box < Sleep > > > , read_wait : Option < Waker > , rx : UnboundedReceiverStream < Action > , name : String , }
    };
}

Inner!()
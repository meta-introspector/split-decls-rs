macro_rules! deps {
    () => {
        NotifyEvent!();
        Message!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        # [derive (Debug)] enum Event { Message (Message) , NotifyEvent (NotifyEvent) , }
    };
}

Event!()
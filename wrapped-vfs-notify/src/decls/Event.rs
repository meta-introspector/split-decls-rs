macro_rules! deps {
    () => {
        Message!();
        NotifyEvent!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        # [derive (Debug)] enum Event { Message (Message) , NotifyEvent (NotifyEvent) , }
    };
}

Event!()
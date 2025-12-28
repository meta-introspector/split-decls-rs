macro_rules! deps {
    () => {
        Event!();
    };
}

macro_rules! NotifyEvent {
    () => {
        deps!();
        type NotifyEvent = notify :: Result < notify :: Event > ;
    };
}

NotifyEvent!()
macro_rules! deps {
    () => {
        NodeStatus!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        struct Event < N > { node : N , becomes : NodeStatus , }
    };
}

Event!()
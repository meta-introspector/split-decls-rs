macro_rules! deps {
    () => {
        EventKind!();
        Span!();
        Encoding!();
    };
}

macro_rules! Event {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub struct Event { kind : EventKind , encoding : Option < Encoding > , span : Span , }
    };
}

Event!()
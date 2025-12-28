macro_rules! deps {
    () => {
        Parent!();
        ValueSet!();
        Metadata!();
        Visit!();
        Id!();
        Current!();
        Iter!();
        Event!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'a > Event < 'a > { # [doc = " Constructs a new `Event` with the specified metadata and set of values,"] # [doc = " and observes it with the current subscriber."] pub fn dispatch (metadata : & 'static Metadata < 'static > , fields : & 'a field :: ValueSet < '_ >) { let event = Event :: new (metadata , fields) ; crate :: dispatcher :: get_default (| current | { current . event (& event) ; }) ; } # [doc = " Returns a new `Event` in the current span, with the specified metadata"] # [doc = " and set of values."] # [inline] pub fn new (metadata : & 'static Metadata < 'static > , fields : & 'a field :: ValueSet < 'a >) -> Self { Event { fields , metadata , parent : Parent :: Current , } } # [doc = " Returns a new `Event` as a child of the specified span, with the"] # [doc = " provided metadata and set of values."] # [inline] pub fn new_child_of (parent : impl Into < Option < Id > > , metadata : & 'static Metadata < 'static > , fields : & 'a field :: ValueSet < 'a > ,) -> Self { let parent = match parent . into () { Some (p) => Parent :: Explicit (p) , None => Parent :: Root , } ; Event { fields , metadata , parent , } } # [doc = " Constructs a new `Event` with the specified metadata and set of values,"] # [doc = " and observes it with the current subscriber and an explicit parent."] pub fn child_of (parent : impl Into < Option < Id > > , metadata : & 'static Metadata < 'static > , fields : & 'a field :: ValueSet < '_ > ,) { let event = Self :: new_child_of (parent , metadata , fields) ; crate :: dispatcher :: get_default (| current | { current . event (& event) ; }) ; } # [doc = " Visits all the fields on this `Event` with the specified [visitor]."] # [doc = ""] # [doc = " [visitor]: super::field::Visit"] # [inline] pub fn record (& self , visitor : & mut dyn field :: Visit) { self . fields . record (visitor) ; } # [doc = " Returns an iterator over the set of values on this `Event`."] pub fn fields (& self) -> field :: Iter { self . fields . field_set () . iter () } # [doc = " Returns [metadata] describing this `Event`."] # [doc = ""] # [doc = " [metadata]: super::Metadata"] pub fn metadata (& self) -> & 'static Metadata < 'static > { self . metadata } # [doc = " Returns true if the new event should be a root."] pub fn is_root (& self) -> bool { matches ! (self . parent , Parent :: Root) } # [doc = " Returns true if the new event's parent should be determined based on the"] # [doc = " current context."] # [doc = ""] # [doc = " If this is true and the current thread is currently inside a span, then"] # [doc = " that span should be the new event's parent. Otherwise, if the current"] # [doc = " thread is _not_ inside a span, then the new event will be the root of its"] # [doc = " own trace tree."] pub fn is_contextual (& self) -> bool { matches ! (self . parent , Parent :: Current) } # [doc = " Returns the new event's explicitly-specified parent, if there is one."] # [doc = ""] # [doc = " Otherwise (if the new event is a root or is a child of the current span),"] # [doc = " returns `None`."] pub fn parent (& self) -> Option < & Id > { match self . parent { Parent :: Explicit (ref p) => Some (p) , _ => None , } } }
    };
}

impl_102!()
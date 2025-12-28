macro_rules! deps {
    () => {
        Current!();
        Subscriber!();
        Id!();
        Metadata!();
        CurrentInner!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl Current { # [doc = " Constructs a new `Current` that indicates the current context is a span"] # [doc = " with the given `metadata` and `metadata`."] pub fn new (id : Id , metadata : & 'static Metadata < 'static >) -> Self { Self { inner : CurrentInner :: Current { id , metadata } , } } # [doc = " Constructs a new `Current` that indicates the current context is *not*"] # [doc = " in a span."] pub fn none () -> Self { Self { inner : CurrentInner :: None , } } # [doc = " Constructs a new `Current` that indicates the `Subscriber` does not"] # [doc = " track a current span."] pub (crate) fn unknown () -> Self { Self { inner : CurrentInner :: Unknown , } } # [doc = " Returns `true` if the `Subscriber` that constructed this `Current` tracks a"] # [doc = " current span."] # [doc = ""] # [doc = " If this returns `true` and [`id`], [`metadata`], or [`into_inner`]"] # [doc = " return `None`, that indicates that we are currently known to *not* be"] # [doc = " inside a span. If this returns `false`, those methods will also return"] # [doc = " `None`, but in this case, that is because the subscriber does not keep"] # [doc = " track of the currently-entered span."] # [doc = ""] # [doc = " [`id`]: Current::id()"] # [doc = " [`metadata`]: Current::metadata()"] # [doc = " [`into_inner`]: Current::into_inner()"] pub fn is_known (& self) -> bool { ! matches ! (self . inner , CurrentInner :: Unknown) } # [doc = " Consumes `self` and returns the span `Id` and `Metadata` of the current"] # [doc = " span, if one exists and is known."] pub fn into_inner (self) -> Option < (Id , & 'static Metadata < 'static >) > { match self . inner { CurrentInner :: Current { id , metadata } => Some ((id , metadata)) , _ => None , } } # [doc = " Borrows the `Id` of the current span, if one exists and is known."] pub fn id (& self) -> Option < & Id > { match self . inner { CurrentInner :: Current { ref id , .. } => Some (id) , _ => None , } } # [doc = " Borrows the `Metadata` of the current span, if one exists and is known."] pub fn metadata (& self) -> Option < & 'static Metadata < 'static > > { match self . inner { CurrentInner :: Current { metadata , .. } => Some (metadata) , _ => None , } } }
    };
}

impl_237!()
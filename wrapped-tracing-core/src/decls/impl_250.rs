macro_rules! deps {
    () => {
        InterestKind!();
        Interest!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl Interest { # [doc = " Returns an `Interest` indicating that the subscriber is never interested"] # [doc = " in being notified about a callsite."] # [doc = ""] # [doc = " If all active subscribers are `never()` interested in a callsite, it will"] # [doc = " be completely disabled unless a new subscriber becomes active."] # [inline] pub fn never () -> Self { Interest (InterestKind :: Never) } # [doc = " Returns an `Interest` indicating the subscriber is sometimes interested"] # [doc = " in being notified about a callsite."] # [doc = ""] # [doc = " If all active subscribers are `sometimes` or `never` interested in a"] # [doc = " callsite, the currently active subscriber will be asked to filter that"] # [doc = " callsite every time it creates a span. This will be the case until a new"] # [doc = " subscriber expresses that it is `always` interested in the callsite."] # [inline] pub fn sometimes () -> Self { Interest (InterestKind :: Sometimes) } # [doc = " Returns an `Interest` indicating the subscriber is always interested in"] # [doc = " being notified about a callsite."] # [doc = ""] # [doc = " If any subscriber expresses that it is `always()` interested in a given"] # [doc = " callsite, then the callsite will always be enabled."] # [inline] pub fn always () -> Self { Interest (InterestKind :: Always) } # [doc = " Returns `true` if the subscriber is never interested in being notified"] # [doc = " about this callsite."] # [inline] pub fn is_never (& self) -> bool { matches ! (self . 0 , InterestKind :: Never) } # [doc = " Returns `true` if the subscriber is sometimes interested in being notified"] # [doc = " about this callsite."] # [inline] pub fn is_sometimes (& self) -> bool { matches ! (self . 0 , InterestKind :: Sometimes) } # [doc = " Returns `true` if the subscriber is always interested in being notified"] # [doc = " about this callsite."] # [inline] pub fn is_always (& self) -> bool { matches ! (self . 0 , InterestKind :: Always) } # [doc = " Returns the common interest between these two Interests."] # [doc = ""] # [doc = " If both interests are the same, this propagates that interest."] # [doc = " Otherwise, if they differ, the result must always be"] # [doc = " `Interest::sometimes` --- if the two subscribers differ in opinion, we"] # [doc = " will have to ask the current subscriber what it thinks, no matter what."] pub (crate) fn and (self , rhs : Interest) -> Self { if self . 0 == rhs . 0 { self } else { Interest :: sometimes () } } }
    };
}

impl_250!();
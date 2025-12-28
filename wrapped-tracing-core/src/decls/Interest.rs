macro_rules! deps {
    () => {
        Subscriber!();
        InterestKind!();
    };
}

macro_rules! Interest {
    () => {
        deps!();
        # [doc = " Indicates a [`Subscriber`]'s interest in a particular callsite."] # [doc = ""] # [doc = " `Subscriber`s return an `Interest` from their [`register_callsite`] methods"] # [doc = " in order to determine whether that span should be enabled or disabled."] # [doc = ""] # [doc = " [`Subscriber`]: super::Subscriber"] # [doc = " [`register_callsite`]: super::Subscriber::register_callsite"] # [derive (Clone , Debug)] pub struct Interest (InterestKind) ;
    };
}

Interest!()
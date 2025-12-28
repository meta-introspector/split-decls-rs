macro_rules! deps {
    () => {
        Subscriber!();
    };
}

macro_rules! NoSubscriber {
    () => {
        deps!();
        # [doc = " A no-op [`Subscriber`]."] # [doc = ""] # [doc = " [`NoSubscriber`] implements the [`Subscriber`] trait by never being enabled,"] # [doc = " never being interested in any callsite, and dropping all spans and events."] # [derive (Copy , Clone , Debug , Default)] pub struct NoSubscriber (()) ;
    };
}

NoSubscriber!();
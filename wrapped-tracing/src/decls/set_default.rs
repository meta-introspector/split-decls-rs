macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! set_default {
    () => {
        deps!();
        # [doc = " Sets the [`Subscriber`] as the default for the current thread for the"] # [doc = " duration of the lifetime of the returned [`DefaultGuard`]."] # [doc = ""] # [doc = " The default subscriber is used when creating a new [`Span`] or [`Event`]."] # [doc = ""] # [doc = " [`Span`]: super::span::Span"] # [doc = " [`Subscriber`]: super::subscriber::Subscriber"] # [doc = " [`Event`]: super::event::Event"] # [doc = " [`DefaultGuard`]: super::dispatcher::DefaultGuard"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] # [must_use = "Dropping the guard unregisters the subscriber."] pub fn set_default < S > (subscriber : S) -> DefaultGuard where S : Subscriber + Send + Sync + 'static , { crate :: dispatcher :: set_default (& crate :: Dispatch :: new (subscriber)) }
    };
}

set_default!();
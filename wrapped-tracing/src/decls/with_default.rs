macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! with_default {
    () => {
        deps!();
        # [doc = " Sets this [`Subscriber`] as the default for the current thread for the"] # [doc = " duration of a closure."] # [doc = ""] # [doc = " The default subscriber is used when creating a new [`Span`] or"] # [doc = " [`Event`]."] # [doc = ""] # [doc = ""] # [doc = " [`Span`]: super::span::Span"] # [doc = " [`Subscriber`]: super::subscriber::Subscriber"] # [doc = " [`Event`]: super::event::Event"] # [cfg (feature = "std")] # [cfg_attr (docsrs , doc (cfg (feature = "std")))] pub fn with_default < T , S > (subscriber : S , f : impl FnOnce () -> T) -> T where S : Subscriber + Send + Sync + 'static , { crate :: dispatcher :: with_default (& crate :: Dispatch :: new (subscriber) , f) }
    };
}

with_default!();
macro_rules! deps {
    () => {
        Subscriber!();
        Callsite!();
    };
}

macro_rules! rebuild_interest_cache {
    () => {
        deps!();
        # [doc = " Clear and reregister interest on every [`Callsite`]"] # [doc = ""] # [doc = " This function is intended for runtime reconfiguration of filters on traces"] # [doc = " when the filter recalculation is much less frequent than trace events are."] # [doc = " The alternative is to have the [`Subscriber`] that supports runtime"] # [doc = " reconfiguration of filters always return [`Interest::sometimes()`] so that"] # [doc = " [`enabled`] is evaluated for every event."] # [doc = ""] # [doc = " This function will also re-compute the global maximum level as determined by"] # [doc = " the [`max_level_hint`] method. If a [`Subscriber`]"] # [doc = " implementation changes the value returned by its `max_level_hint`"] # [doc = " implementation at runtime, then it **must** call this function after that"] # [doc = " value changes, in order for the change to be reflected."] # [doc = ""] # [doc = " See the [documentation on callsite interest caching][cache-docs] for"] # [doc = " additional information on this function's usage."] # [doc = ""] # [doc = " [`max_level_hint`]: super::subscriber::Subscriber::max_level_hint"] # [doc = " [`Callsite`]: super::callsite::Callsite"] # [doc = " [`enabled`]: super::subscriber::Subscriber#tymethod.enabled"] # [doc = " [`Interest::sometimes()`]: super::subscriber::Interest::sometimes"] # [doc = " [`Subscriber`]: super::subscriber::Subscriber"] # [doc = " [cache-docs]: crate::callsite#rebuilding-cached-interest"] pub fn rebuild_interest_cache () { CALLSITES . rebuild_interest (DISPATCHERS . rebuilder ()) ; }
    };
}

rebuild_interest_cache!()
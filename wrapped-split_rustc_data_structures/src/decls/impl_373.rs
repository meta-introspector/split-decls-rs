macro_rules! deps {
    () => {
        SelfProfiler!();
        TimingGuard!();
        QueryInvocationId!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        impl < 'a > TimingGuard < 'a > { # [inline] pub fn start (profiler : & 'a SelfProfiler , event_kind : StringId , event_id : EventId ,) -> TimingGuard < 'a > { let thread_id = get_thread_id () ; let raw_profiler = & profiler . profiler ; let timing_guard = raw_profiler . start_recording_interval_event (event_kind , event_id , thread_id) ; TimingGuard (Some (timing_guard)) } # [inline] pub fn finish_with_query_invocation_id (self , query_invocation_id : QueryInvocationId) { if let Some (guard) = self . 0 { outline (| | { let event_id = StringId :: new_virtual (query_invocation_id . 0) ; let event_id = EventId :: from_virtual (event_id) ; guard . finish_with_override_event_id (event_id) ; }) ; } } # [inline] pub fn none () -> TimingGuard < 'a > { TimingGuard (None) } # [inline (always)] pub fn run < R > (self , f : impl FnOnce () -> R) -> R { let _timer = self ; f () } }
    };
}

impl_373!();
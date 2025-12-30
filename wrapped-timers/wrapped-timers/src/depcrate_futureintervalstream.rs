// Generated macro for IntervalStream (struct)
macro_rules! Depcrate_futureIntervalStream {
() => {
// Module: crate::future
// Provides: {"IntervalStream"}
// Dependencies: {}
# [doc = " A scheduled interval as a `Stream`."] # [doc = ""] # [doc = " See `IntervalStream::new` for scheduling new intervals."] # [doc = ""] # [doc = " Once scheduled, if you want to stop the interval from continuing to fire,"] # [doc = " you can `drop` the stream."] # [doc = ""] # [doc = " An interval stream will never resolve to `Err`."] # [derive (Debug)] # [must_use = "streams do nothing unless polled or spawned"] pub struct IntervalStream { receiver : mpsc :: UnboundedReceiver < () > , _inner : Interval , }
};
}

// Generated macro for impl_44 (impl)
macro_rules! Depcrate_stream_mockimpl_44 {
() => {
// Module: crate::stream_mock
// Provides: {"impl_44"}
// Dependencies: {}
impl < T : Unpin > Stream for StreamMock < T > { type Item = T ; fn poll_next (mut self : std :: pin :: Pin < & mut Self > , cx : & mut std :: task :: Context < '_ > ,) -> std :: task :: Poll < Option < Self :: Item > > { if let Some (ref mut sleep) = self . sleep { ready ! (Pin :: new (sleep) . poll (cx)) ; self . sleep . take () ; } match self . next_action () { Some (action) => match action { Action :: Next (item) => Poll :: Ready (Some (item)) , Action :: Wait (duration) => { self . sleep = Some (Box :: pin (sleep_until (Instant :: now () + duration))) ; cx . waker () . wake_by_ref () ; Poll :: Pending } } , None => Poll :: Ready (None) , } } }
};
}

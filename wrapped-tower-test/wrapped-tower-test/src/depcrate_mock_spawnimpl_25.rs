// Generated macro for impl_25 (impl)
macro_rules! Depcrate_mock_spawnimpl_25 {
() => {
// Module: crate::mock::spawn
// Provides: {"impl_25"}
// Dependencies: {}
impl < T > Spawn < T > { # [doc = " Create a new spawn."] pub fn new (inner : T) -> Self { Self { inner , task : task :: spawn (()) , } } # [doc = " Check if this service has been woken up."] pub fn is_woken (& self) -> bool { self . task . is_woken () } # [doc = " Get how many futurs are holding onto the waker."] pub fn waker_ref_count (& self) -> usize { self . task . waker_ref_count () } # [doc = " Poll this service ready."] pub fn poll_ready < Request > (& mut self) -> Poll < Result < () , T :: Error > > where T : Service < Request > , { let task = & mut self . task ; let inner = & mut self . inner ; task . enter (| cx , _ | inner . poll_ready (cx)) } # [doc = " Call the inner Service."] pub fn call < Request > (& mut self , req : Request) -> T :: Future where T : Service < Request > , { self . inner . call (req) } # [doc = " Get the inner service."] pub fn into_inner (self) -> T { self . inner } # [doc = " Get a reference to the inner service."] pub fn get_ref (& self) -> & T { & self . inner } # [doc = " Get a mutable reference to the inner service."] pub fn get_mut (& mut self) -> & mut T { & mut self . inner } }
};
}

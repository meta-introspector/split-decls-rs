// Generated macro for impl_504 (impl)
macro_rules! Depcrate_threadimpl_504 {
() => {
// Module: crate::thread
// Provides: {"impl_504"}
// Dependencies: {}
impl Thread { # [doc = " Create a new [`Thread`]."] fn new () -> Self { let name = Global :: with (| global | match global { Global :: Dedicated (worker) => Some (worker . name ()) , Global :: Shared (worker) => Some (worker . name ()) , Global :: Window (_) | Global :: Service (_) | Global :: Worklet | Global :: Worker (_) | Global :: Unknown => None , }) . filter (| name | ! name . is_empty ()) ; Self :: new_with_name (name) } # [doc = " Create a new [`Thread`]."] fn new_with_name (name : Option < String >) -> Self { let id = ThreadId :: new () ; Self (Arc :: pin (ThreadInner { id , name , parker : Parker :: new (id) , })) } # [doc = " See [`std::thread::Thread::id()`]."] # [must_use] pub fn id (& self) -> ThreadId { self . 0 . id } # [doc = " See [`std::thread::Thread::name()`]."] # [must_use] pub fn name (& self) -> Option < & str > { self . 0 . name . as_deref () } # [doc = " See [`std::thread::Thread::unpark()`]."] # [inline] pub fn unpark (& self) { Pin :: new (& self . 0 . parker) . unpark () ; } }
};
}

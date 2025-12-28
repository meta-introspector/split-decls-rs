macro_rules! deps {
    () => {
        Thread!();
    };
}

macro_rules! macro_20 {
    () => {
        deps!();
        cfg_if :: cfg_if ! { if # [cfg (feature = "nightly")] { # [thread_local] static mut THREAD : Option < Thread > = None ; thread_local ! { static THREAD_GUARD : ThreadGuard = const { ThreadGuard { id : Cell :: new (0) } } ; } struct ThreadGuard { id : Cell < usize >, } impl Drop for ThreadGuard { fn drop (& mut self) { unsafe { THREAD = None ; } THREAD_ID_MANAGER . lock () . unwrap () . free (self . id . get ()) ; } } # [doc = " Returns a thread ID for the current thread, **not** allocating one if needed."] # [doc = " This avoids registering a thread-local destructor."] # [inline] pub (crate) fn try_get () -> Option < Thread > { unsafe { THREAD } } # [doc = " Returns a thread ID for the current thread, allocating one if needed."] # [inline] pub (crate) fn get () -> Thread { if let Some (thread) = unsafe { THREAD } { thread } else { get_slow () } } # [doc = " Out-of-line slow path for allocating a thread ID."] # [cold] fn get_slow () -> Thread { let new = Thread :: new (THREAD_ID_MANAGER . lock () . unwrap () . alloc ()) ; unsafe { THREAD = Some (new) ; } THREAD_GUARD . with (| guard | guard . id . set (new . id)) ; new } } else { thread_local ! { static THREAD : Cell < Option < Thread >> = const { Cell :: new (None) } ; } thread_local ! { static THREAD_GUARD : ThreadGuard = const { ThreadGuard { id : Cell :: new (0) } } ; } struct ThreadGuard { id : Cell < usize >, } impl Drop for ThreadGuard { fn drop (& mut self) { let _ = THREAD . try_with (| thread | thread . set (None)) ; THREAD_ID_MANAGER . lock () . unwrap () . free (self . id . get ()) ; } } # [doc = " Returns a thread ID for the current thread, **not** allocating one if needed."] # [doc = " This avoids registering a thread-local destructor."] # [inline] pub (crate) fn try_get () -> Option < Thread > { THREAD . with (| thread | thread . get ()) } # [doc = " Returns a thread ID for the current thread, allocating one if needed."] # [inline] pub (crate) fn get () -> Thread { THREAD . with (| thread | { if let Some (thread) = thread . get () { thread } else { get_slow (thread) } }) } # [doc = " Out-of-line slow path for allocating a thread ID."] # [cold] fn get_slow (thread : & Cell < Option < Thread >>) -> Thread { let new = Thread :: new (THREAD_ID_MANAGER . lock () . unwrap () . alloc ()) ; thread . set (Some (new)) ; THREAD_GUARD . with (| guard | guard . id . set (new . id ())) ; new } } }
    };
}

macro_20!()
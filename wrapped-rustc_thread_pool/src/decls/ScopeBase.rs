macro_rules! deps {
    () => {
        Scope!();
        Tlv!();
        Registry!();
        JobRefId!();
        CountLatch!();
    };
}

macro_rules! ScopeBase {
    () => {
        deps!();
        struct ScopeBase < 'scope > { # [doc = " thread registry where `scope()` was executed or where `in_place_scope()`"] # [doc = " should spawn jobs."] registry : Arc < Registry > , # [doc = " if some job panicked, the error is stored here; it will be"] # [doc = " propagated to the one who created the scope"] panic : AtomicPtr < Box < dyn Any + Send + 'static > > , # [doc = " latch to track job counts"] job_completed_latch : CountLatch , # [doc = " Jobs that have been spawned, but not yet started."] # [allow (rustc :: default_hash_types)] pending_jobs : Mutex < HashSet < JobRefId > > , # [doc = " The worker which will wait on scope completion, if any."] worker : Option < usize > , # [doc = " You can think of a scope as containing a list of closures to execute,"] # [doc = " all of which outlive `'scope`. They're not actually required to be"] # [doc = " `Sync`, but it's still safe to let the `Scope` implement `Sync` because"] # [doc = " the closures are only *moved* across threads to be executed."] # [allow (clippy :: type_complexity)] marker : PhantomData < Box < dyn FnOnce (& Scope < 'scope >) + Send + Sync + 'scope > > , # [doc = " The TLV at the scope's creation. Used to set the TLV for spawned jobs."] tlv : Tlv , }
    };
}

ScopeBase!()
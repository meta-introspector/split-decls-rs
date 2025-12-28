macro_rules! deps {
    () => {
        ThreadPoolBuilder!();
        Registry!();
    };
}

macro_rules! spawn_fifo {
    () => {
        deps!();
        # [doc = " Fires off a task into the Rayon threadpool in the \"static\" or"] # [doc = " \"global\" scope. Just like a standard thread, this task is not"] # [doc = " tied to the current stack frame, and hence it cannot hold any"] # [doc = " references other than those with `'static` lifetime. If you want"] # [doc = " to spawn a task that references stack data, use [the `scope_fifo()`"] # [doc = " function](fn.scope_fifo.html) to create a scope."] # [doc = ""] # [doc = " The behavior is essentially the same as [the `spawn`"] # [doc = " function](fn.spawn.html), except that calls from the same thread"] # [doc = " will be prioritized in FIFO order. This is similar to the now-"] # [doc = " deprecated [`breadth_first`] option, except the effect is isolated"] # [doc = " to relative `spawn_fifo` calls, not all threadpool tasks."] # [doc = ""] # [doc = " For more details on this design, see Rayon [RFC #1]."] # [doc = ""] # [doc = " [`breadth_first`]: struct.ThreadPoolBuilder.html#method.breadth_first"] # [doc = " [RFC #1]: https://github.com/rayon-rs/rfcs/blob/master/accepted/rfc0001-scope-scheduling.md"] # [doc = ""] # [doc = " # Panic handling"] # [doc = ""] # [doc = " If this closure should panic, the resulting panic will be"] # [doc = " propagated to the panic handler registered in the `ThreadPoolBuilder`,"] # [doc = " if any. See [`ThreadPoolBuilder::panic_handler()`][ph] for more"] # [doc = " details."] # [doc = ""] # [doc = " [ph]: struct.ThreadPoolBuilder.html#method.panic_handler"] pub fn spawn_fifo < F > (func : F) where F : FnOnce () + Send + 'static , { unsafe { spawn_fifo_in (func , & Registry :: current ()) } }
    };
}

spawn_fifo!()
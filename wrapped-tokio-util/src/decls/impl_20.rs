macro_rules! deps {
    () => {
        JoinMap!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < K , V , S > JoinMap < K , V , S > where V : 'static , { # [doc = " Aborts all tasks on this `JoinMap`."] # [doc = ""] # [doc = " This does not remove the tasks from the `JoinMap`. To wait for the tasks to complete"] # [doc = " cancellation, you should call `join_next` in a loop until the `JoinMap` is empty."] pub fn abort_all (& mut self) { self . tasks . abort_all () } # [doc = " Removes all tasks from this `JoinMap` without aborting them."] # [doc = ""] # [doc = " The tasks removed by this call will continue to run in the background even if the `JoinMap`"] # [doc = " is dropped. They may still be aborted by key."] pub fn detach_all (& mut self) { self . tasks . detach_all () ; self . tasks_by_key . clear () ; self . hashes_by_task . clear () ; } }
    };
}

impl_20!();
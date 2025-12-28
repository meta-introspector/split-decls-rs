macro_rules! WorkerSleepState {
    () => {
        # [doc = " The \"sleep state\" for an individual worker."] # [derive (Default)] struct WorkerSleepState { # [doc = " Set to true when the worker goes to sleep; set to false when"] # [doc = " the worker is notified or when it wakes."] is_blocked : Mutex < bool > , condvar : Condvar , }
    };
}

WorkerSleepState!()
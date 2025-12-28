macro_rules! CoreSchedulingScope {
    () => {
        # [doc = " `PR_SCHED_CORE_SCOPE_*`"] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum CoreSchedulingScope { # [doc = " Operation will be performed for the thread."] Thread = PR_SCHED_CORE_SCOPE_THREAD , # [doc = " Operation will be performed for all tasks in the task group of the"] # [doc = " process."] ThreadGroup = PR_SCHED_CORE_SCOPE_THREAD_GROUP , # [doc = " Operation will be performed for all processes in the process group."] ProcessGroup = PR_SCHED_CORE_SCOPE_PROCESS_GROUP , }
    };
}

CoreSchedulingScope!();
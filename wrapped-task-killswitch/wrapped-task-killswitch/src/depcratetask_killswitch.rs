// Generated macro for TASK_KILLSWITCH (static)
macro_rules! DepcrateTASK_KILLSWITCH {
() => {
// Module: crate
// Provides: {"TASK_KILLSWITCH"}
// Dependencies: {}
# [doc = " The global [`TaskKillswitch`] exposed publicly from the crate."] static TASK_KILLSWITCH : LazyLock < TaskKillswitch > = LazyLock :: new (TaskKillswitch :: with_leaked_storage) ;
};
}

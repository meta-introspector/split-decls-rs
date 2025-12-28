macro_rules! deps {
    () => {
        ThreadWaker!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl ThreadWaker { fn new () -> Self { ThreadWaker { state : Mutex :: new (IDLE) , condvar : Condvar :: new () , } } # [doc = " Clears any previously received wakes, avoiding potential spurious"] # [doc = " wake notifications. This should only be called immediately before running the"] # [doc = " task."] fn clear (& self) { * self . state . lock () . unwrap () = IDLE ; } fn is_woken (& self) -> bool { match * self . state . lock () . unwrap () { IDLE => false , WAKE => true , _ => unreachable ! () , } } fn wake (& self) { let mut state = self . state . lock () . unwrap () ; let prev = * state ; if prev == WAKE { return ; } * state = WAKE ; if prev == IDLE { return ; } assert_eq ! (prev , SLEEP) ; self . condvar . notify_one () ; } }
    };
}

impl_51!()
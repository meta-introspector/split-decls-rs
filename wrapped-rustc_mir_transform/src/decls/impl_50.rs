macro_rules! deps {
    () => {
        Unwind!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl Unwind { fn is_cleanup (self) -> bool { match self { Unwind :: To (..) => false , Unwind :: InCleanup => true , } } fn into_action (self) -> UnwindAction { match self { Unwind :: To (bb) => UnwindAction :: Cleanup (bb) , Unwind :: InCleanup => UnwindAction :: Terminate (UnwindTerminateReason :: InCleanup) , } } fn map < F > (self , f : F) -> Self where F : FnOnce (BasicBlock) -> BasicBlock , { match self { Unwind :: To (bb) => Unwind :: To (f (bb)) , Unwind :: InCleanup => Unwind :: InCleanup , } } }
    };
}

impl_50!()
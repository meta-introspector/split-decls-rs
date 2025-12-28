macro_rules! deps {
    () => {
        PTP_CLEANUP_GROUP_CANCEL_CALLBACK!();
        TP_CALLBACK_PRIORITY!();
        Pool!();
        PTP_SIMPLE_CALLBACK!();
        PTP_POOL!();
        PTP_CLEANUP_GROUP!();
    };
}

macro_rules! TP_CALLBACK_ENVIRON_V3 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub struct TP_CALLBACK_ENVIRON_V3 { pub Version : u32 , pub Pool : PTP_POOL , pub CleanupGroup : PTP_CLEANUP_GROUP , pub CleanupGroupCancelCallback : PTP_CLEANUP_GROUP_CANCEL_CALLBACK , pub RaceDll : * mut core :: ffi :: c_void , pub ActivationContext : isize , pub FinalizationCallback : PTP_SIMPLE_CALLBACK , pub u : TP_CALLBACK_ENVIRON_V3_0 , pub CallbackPriority : TP_CALLBACK_PRIORITY , pub Size : u32 , }
    };
}

TP_CALLBACK_ENVIRON_V3!()
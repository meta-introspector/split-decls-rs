macro_rules! deps {
    () => {
        Target!();
        TargetOptions!();
    };
}

macro_rules! aarch64 {
    () => {
        deps!();
        pub (crate) fn aarch64 () -> Target { Target { llvm_target : "aarch64-unknown-unknown" . into () , metadata : meta () , pointer_width : 64 , data_layout : "e-m:e-p270:32:32-p271:32:32-p272:64:64-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128-Fn32" . into () , arch : "aarch64" . into () , options : TargetOptions { features : "+v8a" . into () , max_atomic_width : Some (128) , .. opts () } } }
    };
}

aarch64!();
macro_rules! deps {
    () => {
        TargetOptions!();
        Target!();
    };
}

macro_rules! x86_64 {
    () => {
        deps!();
        pub (crate) fn x86_64 () -> Target { Target { llvm_target : "x86_64-pc-unknown" . into () , metadata : meta () , pointer_width : 64 , data_layout : "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128" . into () , arch : "x86_64" . into () , options : TargetOptions { cpu : "x86-64" . into () , plt_by_default : false , max_atomic_width : Some (64) , vendor : "pc" . into () , .. opts () } , } }
    };
}

x86_64!()
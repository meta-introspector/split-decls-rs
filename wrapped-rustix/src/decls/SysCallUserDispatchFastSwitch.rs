macro_rules! SysCallUserDispatchFastSwitch {
    () => {
        # [doc = " Value of the fast switch flag controlling system calls user dispatch"] # [doc = " mechanism without the need to issue a syscall."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u8)] pub enum SysCallUserDispatchFastSwitch { # [doc = " System calls are allowed to execute."] Allow = SYSCALL_DISPATCH_FILTER_ALLOW , # [doc = " System calls are blocked from executing."] Block = SYSCALL_DISPATCH_FILTER_BLOCK , }
    };
}

SysCallUserDispatchFastSwitch!();
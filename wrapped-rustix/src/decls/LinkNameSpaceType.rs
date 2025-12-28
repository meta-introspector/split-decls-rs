macro_rules! LinkNameSpaceType {
    () => {
        # [doc = " Type of name space referred to by a link."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [repr (u32)] pub enum LinkNameSpaceType { # [doc = " Time name space."] Time = CLONE_NEWTIME , # [doc = " Mount name space."] Mount = CLONE_NEWNS , # [doc = " Control group (CGroup) name space."] ControlGroup = CLONE_NEWCGROUP , # [doc = " `Host name` and `NIS domain name` (UTS) name space."] HostNameAndNISDomainName = CLONE_NEWUTS , # [doc = " Inter-process communication (IPC) name space."] InterProcessCommunication = CLONE_NEWIPC , # [doc = " User name space."] User = CLONE_NEWUSER , # [doc = " Process ID name space."] ProcessID = CLONE_NEWPID , # [doc = " Network name space."] Network = CLONE_NEWNET , }
    };
}

LinkNameSpaceType!();
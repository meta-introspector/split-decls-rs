macro_rules! deps {
    () => {
        AncillaryIter!();
        UCred!();
    };
}

macro_rules! RecvAncillaryMessage {
    () => {
        deps!();
        # [doc = " Ancillary message for [`recvmsg`]."] # [non_exhaustive] pub enum RecvAncillaryMessage < 'a > { # [doc = " Received file descriptors."] # [doc (alias = "SCM_RIGHTS")] ScmRights (AncillaryIter < 'a , OwnedFd >) , # [doc = " Received process credentials."] # [cfg (linux_kernel)] # [doc (alias = "SCM_CREDENTIALS")] ScmCredentials (UCred) , }
    };
}

RecvAncillaryMessage!();
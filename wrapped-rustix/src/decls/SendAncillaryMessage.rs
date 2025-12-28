macro_rules! deps {
    () => {
        UCred!();
    };
}

macro_rules! SendAncillaryMessage {
    () => {
        deps!();
        # [doc = " Ancillary message for [`sendmsg`] and [`sendmsg_addr`]."] # [non_exhaustive] pub enum SendAncillaryMessage < 'slice , 'fd > { # [doc = " Send file descriptors."] # [doc (alias = "SCM_RIGHTS")] ScmRights (& 'slice [BorrowedFd < 'fd >]) , # [doc = " Send process credentials."] # [cfg (linux_kernel)] # [doc (alias = "SCM_CREDENTIAL")] ScmCredentials (UCred) , # [doc = " Transmission time, in nanoseconds. The value will be interpreted by"] # [doc = " whichever clock was configured on the socket with [`set_txtime`]."] # [doc = ""] # [doc = " [`set_txtime`]: crate::net::sockopt::set_txtime"] # [cfg (target_os = "linux")] # [doc (alias = "SCM_TXTIME")] TxTime (u64) , }
    };
}

SendAncillaryMessage!()
macro_rules! deps {
    () => {
        Uid!();
        Gid!();
        Pid!();
    };
}

macro_rules! UCred {
    () => {
        deps!();
        # [doc = " UNIX credentials of socket peer, for use with [`get_socket_peercred`]"] # [doc = " [`SendAncillaryMessage::ScmCredentials`] and"] # [doc = " [`RecvAncillaryMessage::ScmCredentials`]."] # [doc = ""] # [doc = " [`get_socket_peercred`]: crate::net::sockopt::socket_peercred"] # [doc = " [`SendAncillaryMessage::ScmCredentials`]: crate::net::SendAncillaryMessage::ScmCredentials"] # [doc = " [`RecvAncillaryMessage::ScmCredentials`]: crate::net::RecvAncillaryMessage::ScmCredentials"] # [cfg (linux_kernel)] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (C)] pub struct UCred { # [doc = " Process ID of peer"] pub pid : crate :: pid :: Pid , # [doc = " User ID of peer"] pub uid : crate :: ugid :: Uid , # [doc = " Group ID of peer"] pub gid : crate :: ugid :: Gid , }
    };
}

UCred!();
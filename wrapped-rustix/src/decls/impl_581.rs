macro_rules! deps {
    () => {
        AncillaryDrain!();
        RecvAncillaryMessage!();
        AncillaryIter!();
        UCred!();
        RecvAncillaryBuffer!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl < 'buf > AncillaryDrain < 'buf > { # [doc = " Create an iterator for control messages that were received without"] # [doc = " [`RecvAncillaryBuffer`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The buffer must contain valid message data (or be empty)."] pub unsafe fn parse (buffer : & 'buf mut [u8]) -> Self { Self { messages : messages :: Messages :: new (buffer) , read_and_length : None , } } fn advance (read_and_length : & mut Option < (& 'buf mut usize , & 'buf mut usize) > , msg : & c :: cmsghdr ,) -> Option < RecvAncillaryMessage < 'buf > > { if let Some ((read , length)) = read_and_length { let msg_len = msg . cmsg_len as usize ; * * read += msg_len ; * * length -= msg_len ; } Self :: cvt_msg (msg) } # [doc = " A closure that converts a message into a [`RecvAncillaryMessage`]."] fn cvt_msg (msg : & c :: cmsghdr) -> Option < RecvAncillaryMessage < 'buf > > { unsafe { let payload = c :: CMSG_DATA (msg) ; let payload_len = msg . cmsg_len as usize - c :: CMSG_LEN (0) as usize ; let payload : & 'buf mut [u8] = slice :: from_raw_parts_mut (payload , payload_len) ; let (level , msg_type) = (msg . cmsg_level , msg . cmsg_type) ; match (level as _ , msg_type as _) { (c :: SOL_SOCKET , c :: SCM_RIGHTS) => { let fds = AncillaryIter :: new (payload) ; Some (RecvAncillaryMessage :: ScmRights (fds)) } # [cfg (linux_kernel)] (c :: SOL_SOCKET , c :: SCM_CREDENTIALS) => { if payload_len >= size_of :: < UCred > () { let ucred = payload . as_ptr () . cast :: < UCred > () . read_unaligned () ; Some (RecvAncillaryMessage :: ScmCredentials (ucred)) } else { None } } _ => None , } } } }
    };
}

impl_581!()
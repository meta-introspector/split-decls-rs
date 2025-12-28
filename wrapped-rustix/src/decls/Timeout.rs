macro_rules! Timeout {
    () => {
        # [doc = " Timeout identifier for use with [`set_socket_timeout`] and"] # [doc = " [`socket_timeout`]."] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u32)] pub enum Timeout { # [doc = " `SO_RCVTIMEO`—Timeout for receiving."] Recv = c :: SO_RCVTIMEO as _ , # [doc = " `SO_SNDTIMEO`—Timeout for sending."] Send = c :: SO_SNDTIMEO as _ , }
    };
}

Timeout!();
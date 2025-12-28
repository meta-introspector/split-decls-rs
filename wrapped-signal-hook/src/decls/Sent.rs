macro_rules! Sent {
    () => {
        # [doc = " The means by which a signal was sent by other process."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] # [non_exhaustive] pub enum Sent { # [doc = " The `kill` call."] User , # [doc = " The `tkill` call."] # [doc = ""] # [doc = " This is likely linux specific."] TKill , # [doc = " `sigqueue`."] Queue , # [doc = " `mq_notify`."] MesgQ , }
    };
}

Sent!()
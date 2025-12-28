macro_rules! QueueSelector {
    () => {
        # [doc = " `TC*` values for use with [`tcflush`]."] # [doc = ""] # [doc = " [`tcflush`]: crate::termios::tcflush"] # [derive (Debug , Clone , Copy , Eq , PartialEq , Hash)] # [repr (u32)] pub enum QueueSelector { # [doc = " `TCIFLUSH`—Flush data received but not read."] # [doc (alias = "TCIFLUSH")] IFlush = c :: TCIFLUSH as u32 , # [doc = " `TCOFLUSH`—Flush data written but not transmitted."] # [doc (alias = "TCOFLUSH")] OFlush = c :: TCOFLUSH as u32 , # [doc = " `TCIOFLUSH`—`IFlush` and `OFlush` combined."] # [doc (alias = "TCIOFLUSH")] IOFlush = c :: TCIOFLUSH as u32 , }
    };
}

QueueSelector!()
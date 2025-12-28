macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! BE {
    () => {
        deps!();
        # [doc = " A type alias for [`BigEndian`]."] pub type BE = BigEndian ;
    };
}

BE!();
macro_rules! deps {
    () => {
        SendAncillaryMessage!();
        SendAncillaryBuffer!();
    };
}

macro_rules! impl_574 {
    () => {
        deps!();
        impl < 'slice , 'fd > Extend < SendAncillaryMessage < 'slice , 'fd > > for SendAncillaryBuffer < '_ , 'slice , 'fd > { fn extend < T : IntoIterator < Item = SendAncillaryMessage < 'slice , 'fd > > > (& mut self , iter : T) { iter . into_iter () . all (| msg | self . push (msg)) ; } }
    };
}

impl_574!();
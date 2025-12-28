macro_rules! deps {
    () => {
        IntoBytes!();
        FromBytes!();
        SliceDst!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        # [allow (clippy :: must_use_candidate , clippy :: missing_inline_in_public_items , clippy :: todo)] impl < T : FromBytes + IntoBytes , U : FromBytes + IntoBytes > SliceDst < T , U > { pub fn new () -> & 'static SliceDst < T , U > { todo ! () } pub fn new_mut () -> & 'static mut SliceDst < T , U > { todo ! () } }
    };
}

impl_172!();
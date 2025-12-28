macro_rules! deps {
    () => {
        FollowerSet!();
    };
}

macro_rules! FollowerSetArray {
    () => {
        deps!();
        # [doc = " Read the follower sets from is into fsets. Returns true on success."] type FollowerSetArray = [FollowerSet ; u8 :: MAX as usize + 1] ;
    };
}

FollowerSetArray!()
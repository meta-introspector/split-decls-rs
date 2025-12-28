macro_rules! FollowerSet {
    () => {
        # [derive (Default , Clone , Copy)] struct FollowerSet { followers : [u8 ; 32] , size : u8 , idx_bitlen : u8 , }
    };
}

FollowerSet!();
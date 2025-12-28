macro_rules! deps {
    () => {
        ExpnId!();
        HygieneData!();
        ExpnData!();
        ExpnHash!();
    };
}

macro_rules! decode_expn_id {
    () => {
        deps!();
        # [doc = " Decode an expansion from the metadata of a foreign crate."] pub fn decode_expn_id (krate : CrateNum , index : u32 , decode_data : impl FnOnce (ExpnId) -> (ExpnData , ExpnHash) ,) -> ExpnId { if index == 0 { trace ! ("decode_expn_id: deserialized root") ; return ExpnId :: root () ; } let index = ExpnIndex :: from_u32 (index) ; debug_assert_ne ! (krate , LOCAL_CRATE) ; let expn_id = ExpnId { krate , local_id : index } ; if HygieneData :: with (| hygiene_data | hygiene_data . foreign_expn_data . contains_key (& expn_id)) { return expn_id ; } let (expn_data , hash) = decode_data (expn_id) ; register_expn_id (krate , index , expn_data , hash) }
    };
}

decode_expn_id!();
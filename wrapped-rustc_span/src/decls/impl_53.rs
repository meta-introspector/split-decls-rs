macro_rules! deps {
    () => {
        HashStableContext!();
        ExpnHash!();
        HygieneData!();
        ExpnId!();
        Span!();
        ExpnData!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl LocalExpnId { # [doc = " The ID of the theoretical expansion that generates freshly parsed, unexpanded AST."] pub const ROOT : LocalExpnId = LocalExpnId :: ZERO ; # [inline] fn from_raw (idx : ExpnIndex) -> LocalExpnId { LocalExpnId :: from_u32 (idx . as_u32 ()) } # [inline] pub fn as_raw (self) -> ExpnIndex { ExpnIndex :: from_u32 (self . as_u32 ()) } pub fn fresh_empty () -> LocalExpnId { HygieneData :: with (| data | { let expn_id = data . local_expn_data . push (None) ; let _eid = data . local_expn_hashes . push (ExpnHash (Fingerprint :: ZERO)) ; debug_assert_eq ! (expn_id , _eid) ; expn_id }) } pub fn fresh (mut expn_data : ExpnData , ctx : impl HashStableContext) -> LocalExpnId { debug_assert_eq ! (expn_data . parent . krate , LOCAL_CRATE) ; let expn_hash = update_disambiguator (& mut expn_data , ctx) ; HygieneData :: with (| data | { let expn_id = data . local_expn_data . push (Some (expn_data)) ; let _eid = data . local_expn_hashes . push (expn_hash) ; debug_assert_eq ! (expn_id , _eid) ; let _old_id = data . expn_hash_to_expn_id . insert (expn_hash , expn_id . to_expn_id ()) ; debug_assert ! (_old_id . is_none ()) ; expn_id }) } # [inline] pub fn expn_data (self) -> ExpnData { HygieneData :: with (| data | data . local_expn_data (self) . clone ()) } # [inline] pub fn to_expn_id (self) -> ExpnId { ExpnId { krate : LOCAL_CRATE , local_id : self . as_raw () } } # [inline] pub fn set_expn_data (self , mut expn_data : ExpnData , ctx : impl HashStableContext) { debug_assert_eq ! (expn_data . parent . krate , LOCAL_CRATE) ; let expn_hash = update_disambiguator (& mut expn_data , ctx) ; HygieneData :: with (| data | { let old_expn_data = & mut data . local_expn_data [self] ; assert ! (old_expn_data . is_none () , "expansion data is reset for an expansion ID") ; * old_expn_data = Some (expn_data) ; debug_assert_eq ! (data . local_expn_hashes [self] . 0 , Fingerprint :: ZERO) ; data . local_expn_hashes [self] = expn_hash ; let _old_id = data . expn_hash_to_expn_id . insert (expn_hash , self . to_expn_id ()) ; debug_assert ! (_old_id . is_none ()) ; }) ; } # [inline] pub fn is_descendant_of (self , ancestor : LocalExpnId) -> bool { self . to_expn_id () . is_descendant_of (ancestor . to_expn_id ()) } # [doc = " Returns span for the macro which originally caused this expansion to happen."] # [doc = ""] # [doc = " Stops backtracing at include! boundary."] # [inline] pub fn expansion_cause (self) -> Option < Span > { self . to_expn_id () . expansion_cause () } }
    };
}

impl_53!()
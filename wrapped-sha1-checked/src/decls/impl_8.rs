macro_rules! deps {
    () => {
        Sha1!();
        Builder!();
        CollisionResult!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Sha1 { # [doc = " Create a new Sha1 instance, with collision detection enabled."] pub fn new () -> Self { Self :: default () } # [doc = " Create a new Sha1 builder to configure detection."] pub fn builder () -> Builder { Builder :: default () } # [doc = " Oneshot hashing, reporting the collision state."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hex_literal::hex;"] # [doc = " use sha1_checked::Sha1;"] # [doc = ""] # [doc = " let result = Sha1::try_digest(b\"hello world\");"] # [doc = " assert_eq!(result.hash().as_ref(), hex!(\"2aae6c35c94fcfb415dbe95f408b9ce91ee846ed\"));"] # [doc = " assert!(!result.has_collision());"] # [doc = " ```"] pub fn try_digest (data : impl AsRef < [u8] >) -> CollisionResult { let mut hasher = Self :: default () ; Digest :: update (& mut hasher , data) ; hasher . try_finalize () } # [doc = " Try finalization, reporting the collision state."] pub fn try_finalize (mut self) -> CollisionResult { let mut out = Output :: < Self > :: default () ; self . finalize_inner (& mut out) ; if let Some (ref ctx) = self . detection { if ctx . found_collision { if ctx . safe_hash { return CollisionResult :: Mitigated (out) ; } return CollisionResult :: Collision (out) ; } } CollisionResult :: Ok (out) } fn finalize_inner (& mut self , out : & mut Output < Self >) { let bs = 64 ; let buffer = & mut self . buffer ; let h = & mut self . h ; if let Some (ref mut ctx) = self . detection { let last_block = buffer . get_data () ; compress :: finalize (h , bs * self . block_len , last_block , ctx) ; } else { let bit_len = 8 * (buffer . get_pos () as u64 + bs * self . block_len) ; buffer . len64_padding_be (bit_len , | b | { sha1 :: block_api :: compress (h , from_ref (b . into ())) }) ; } for (chunk , v) in out . chunks_exact_mut (4) . zip (h . iter ()) { chunk . copy_from_slice (& v . to_be_bytes ()) ; } } }
    };
}

impl_8!()
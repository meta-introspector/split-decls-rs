use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Sha1 {
    /// Create a new Sha1 instance, with collision detection enabled.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a new Sha1 builder to configure detection.
    pub fn builder() -> Builder {
        Builder::default()
    }
    /// Oneshot hashing, reporting the collision state.
    ///
    /// # Examples
    ///
    /// ```
    /// use hex_literal::hex;
    /// use sha1_checked::Sha1;
    ///
    /// let result = Sha1::try_digest(b"hello world");
    /// assert_eq!(result.hash().as_ref(), hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
    /// assert!(!result.has_collision());
    /// ```
    pub fn try_digest(data: impl AsRef<[u8]>) -> CollisionResult {
        let mut hasher = Self::default();
        Digest::update(&mut hasher, data);
        hasher.try_finalize()
    }
    /// Try finalization, reporting the collision state.
    pub fn try_finalize(mut self) -> CollisionResult {
        let mut out = Output::<Self>::default();
        self.finalize_inner(&mut out);
        if let Some(ref ctx) = self.detection {
            if ctx.found_collision {
                if ctx.safe_hash {
                    return CollisionResult::Mitigated(out);
                }
                return CollisionResult::Collision(out);
            }
        }
        CollisionResult::Ok(out)
    }
    fn finalize_inner(&mut self, out: &mut Output<Self>) {
        let bs = 64;
        let buffer = &mut self.buffer;
        let h = &mut self.h;
        if let Some(ref mut ctx) = self.detection {
            let last_block = buffer.get_data();
            compress::finalize(h, bs * self.block_len, last_block, ctx);
        } else {
            let bit_len = 8 * (buffer.get_pos() as u64 + bs * self.block_len);
            buffer.len64_padding_be(bit_len, |b| {
                sha1::block_api::compress(h, from_ref(b.into()))
            });
        }
        for (chunk, v) in out.chunks_exact_mut(4).zip(h.iter()) {
            chunk.copy_from_slice(&v.to_be_bytes());
        }
    }
}

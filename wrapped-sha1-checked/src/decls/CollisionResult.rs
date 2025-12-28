macro_rules! deps {
    () => {
        Sha1!();
    };
}

macro_rules! CollisionResult {
    () => {
        deps!();
        # [doc = " Result when trying to finalize a hash."] # [derive (Debug)] pub enum CollisionResult { # [doc = " No collision."] Ok (Output < Sha1 >) , # [doc = " Collision occurred, but was mititgated."] Mitigated (Output < Sha1 >) , # [doc = " Collision occurred, the hash is the one that collided."] Collision (Output < Sha1 >) , }
    };
}

CollisionResult!();
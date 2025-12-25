use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SourceFileHash {
    pub fn new_in_memory(kind: SourceFileHashAlgorithm, src: impl AsRef<[u8]>) -> SourceFileHash {
        let mut hash = SourceFileHash {
            kind,
            value: Default::default(),
        };
        let len = hash.hash_len();
        let value = &mut hash.value[..len];
        let data = src.as_ref();
        match kind {
            SourceFileHashAlgorithm::Md5 => {
                value.copy_from_slice(&Md5::digest(data));
            }
            SourceFileHashAlgorithm::Sha1 => {
                value.copy_from_slice(&Sha1::digest(data));
            }
            SourceFileHashAlgorithm::Sha256 => {
                value.copy_from_slice(&Sha256::digest(data));
            }
            SourceFileHashAlgorithm::Blake3 => value.copy_from_slice(blake3::hash(data).as_bytes()),
        };
        hash
    }
    pub fn new(kind: SourceFileHashAlgorithm, src: impl Read) -> Result<SourceFileHash, io::Error> {
        let mut hash = SourceFileHash {
            kind,
            value: Default::default(),
        };
        let len = hash.hash_len();
        let value = &mut hash.value[..len];
        let mut buf = vec![0; 16 * 1024];
        fn digest<T>(
            mut hasher: T,
            mut update: impl FnMut(&mut T, &[u8]),
            finish: impl FnOnce(T, &mut [u8]),
            mut src: impl Read,
            buf: &mut [u8],
            value: &mut [u8],
        ) -> Result<(), io::Error> {
            loop {
                let bytes_read = src.read(buf)?;
                if bytes_read == 0 {
                    break;
                }
                update(&mut hasher, &buf[0..bytes_read]);
            }
            finish(hasher, value);
            Ok(())
        }
        match kind {
            SourceFileHashAlgorithm::Sha256 => {
                digest(
                    Sha256::new(),
                    |h, b| {
                        h.update(b);
                    },
                    |h, out| out.copy_from_slice(&h.finalize()),
                    src,
                    &mut buf,
                    value,
                )?;
            }
            SourceFileHashAlgorithm::Sha1 => {
                digest(
                    Sha1::new(),
                    |h, b| {
                        h.update(b);
                    },
                    |h, out| out.copy_from_slice(&h.finalize()),
                    src,
                    &mut buf,
                    value,
                )?;
            }
            SourceFileHashAlgorithm::Md5 => {
                digest(
                    Md5::new(),
                    |h, b| {
                        h.update(b);
                    },
                    |h, out| out.copy_from_slice(&h.finalize()),
                    src,
                    &mut buf,
                    value,
                )?;
            }
            SourceFileHashAlgorithm::Blake3 => {
                digest(
                    blake3::Hasher::new(),
                    |h, b| {
                        h.update(b);
                    },
                    |h, out| out.copy_from_slice(h.finalize().as_bytes()),
                    src,
                    &mut buf,
                    value,
                )?;
            }
        }
        Ok(hash)
    }
    /// Check if the stored hash matches the hash of the string.
    pub fn matches(&self, src: &str) -> bool {
        Self::new_in_memory(self.kind, src.as_bytes()) == *self
    }
    /// The bytes of the hash.
    pub fn hash_bytes(&self) -> &[u8] {
        let len = self.hash_len();
        &self.value[..len]
    }
    fn hash_len(&self) -> usize {
        match self.kind {
            SourceFileHashAlgorithm::Md5 => 16,
            SourceFileHashAlgorithm::Sha1 => 20,
            SourceFileHashAlgorithm::Sha256 | SourceFileHashAlgorithm::Blake3 => 32,
        }
    }
}

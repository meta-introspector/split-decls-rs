use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Uuid {
    /// UUID namespace for Domain Name System (DNS).
    pub const NAMESPACE_DNS: Self = Uuid([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
    /// UUID namespace for ISO Object Identifiers (OIDs).
    pub const NAMESPACE_OID: Self = Uuid([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
    /// UUID namespace for Uniform Resource Locators (URLs).
    pub const NAMESPACE_URL: Self = Uuid([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
    /// UUID namespace for X.500 Distinguished Names (DNs).
    pub const NAMESPACE_X500: Self = Uuid([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30,
        0xc8,
    ]);
    /// Returns the variant of the UUID structure.
    ///
    /// This determines the interpretation of the structure of the UUID.
    /// This method simply reads the value of the variant byte. It doesn't
    /// validate the rest of the UUID as conforming to that variant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::{Uuid, Variant};
    /// # fn main() -> Result<(), uuid::Error> {
    /// let my_uuid = Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208")?;
    ///
    /// assert_eq!(Variant::RFC4122, my_uuid.get_variant());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # References
    ///
    /// * [Variant Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.1)
    pub const fn get_variant(&self) -> Variant {
        match self.as_bytes()[8] {
            x if x & 0x80 == 0x00 => Variant::NCS,
            x if x & 0xc0 == 0x80 => Variant::RFC4122,
            x if x & 0xe0 == 0xc0 => Variant::Microsoft,
            x if x & 0xe0 == 0xe0 => Variant::Future,
            _ => Variant::Future,
        }
    }
    /// Returns the version number of the UUID.
    ///
    /// This represents the algorithm used to generate the value.
    /// This method is the future-proof alternative to [`Uuid::get_version`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let my_uuid = Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208")?;
    ///
    /// assert_eq!(3, my_uuid.get_version_num());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # References
    ///
    /// * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)
    pub const fn get_version_num(&self) -> usize {
        (self.as_bytes()[6] >> 4) as usize
    }
    /// Returns the version of the UUID.
    ///
    /// This represents the algorithm used to generate the value.
    /// If the version field doesn't contain a recognized version then `None`
    /// is returned. If you're trying to read the version for a future extension
    /// you can also use [`Uuid::get_version_num`] to unconditionally return a
    /// number. Future extensions may start to return `Some` once they're
    /// standardized and supported.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use uuid::{Uuid, Version};
    /// # fn main() -> Result<(), uuid::Error> {
    /// let my_uuid = Uuid::parse_str("02f09a3f-1624-3b1d-8409-44eff7708208")?;
    ///
    /// assert_eq!(Some(Version::Md5), my_uuid.get_version());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # References
    ///
    /// * [Version Field in RFC 9562](https://www.ietf.org/rfc/rfc9562.html#section-4.2)
    pub const fn get_version(&self) -> Option<Version> {
        match self.get_version_num() {
            0 if self.is_nil() => Some(Version::Nil),
            1 => Some(Version::Mac),
            2 => Some(Version::Dce),
            3 => Some(Version::Md5),
            4 => Some(Version::Random),
            5 => Some(Version::Sha1),
            6 => Some(Version::SortMac),
            7 => Some(Version::SortRand),
            8 => Some(Version::Custom),
            0xf => Some(Version::Max),
            _ => None,
        }
    }
    /// Returns the four field values of the UUID.
    ///
    /// These values can be passed to the [`Uuid::from_fields`] method to get
    /// the original `Uuid` back.
    ///
    /// * The first field value represents the first group of (eight) hex
    ///   digits, taken as a big-endian `u32` value.  For V1 UUIDs, this field
    ///   represents the low 32 bits of the timestamp.
    /// * The second field value represents the second group of (four) hex
    ///   digits, taken as a big-endian `u16` value.  For V1 UUIDs, this field
    ///   represents the middle 16 bits of the timestamp.
    /// * The third field value represents the third group of (four) hex digits,
    ///   taken as a big-endian `u16` value.  The 4 most significant bits give
    ///   the UUID version, and for V1 UUIDs, the last 12 bits represent the
    ///   high 12 bits of the timestamp.
    /// * The last field value represents the last two groups of four and twelve
    ///   hex digits, taken in order.  The first 1-3 bits of this indicate the
    ///   UUID variant, and for V1 UUIDs, the next 13-15 bits indicate the clock
    ///   sequence and the last 48 bits indicate the node ID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::nil();
    ///
    /// assert_eq!(uuid.as_fields(), (0, 0, 0, &[0u8; 8]));
    ///
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.as_fields(),
    ///     (
    ///         0xa1a2a3a4,
    ///         0xb1b2,
    ///         0xc1c2,
    ///         &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8],
    ///     )
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_fields(&self) -> (u32, u16, u16, &[u8; 8]) {
        let bytes = self.as_bytes();
        let d1 = (bytes[0] as u32) << 24
            | (bytes[1] as u32) << 16
            | (bytes[2] as u32) << 8
            | (bytes[3] as u32);
        let d2 = (bytes[4] as u16) << 8 | (bytes[5] as u16);
        let d3 = (bytes[6] as u16) << 8 | (bytes[7] as u16);
        let d4: &[u8; 8] = bytes[8..16].try_into().unwrap();
        (d1, d2, d3, d4)
    }
    /// Returns the four field values of the UUID in little-endian order.
    ///
    /// The bytes in the returned integer fields will be converted from
    /// big-endian order. This is based on the endianness of the UUID,
    /// rather than the target environment so bytes will be flipped on both
    /// big and little endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.to_fields_le(),
    ///     (
    ///         0xa4a3a2a1,
    ///         0xb2b1,
    ///         0xc2c1,
    ///         &[0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8],
    ///     )
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn to_fields_le(&self) -> (u32, u16, u16, &[u8; 8]) {
        let d1 = (self.as_bytes()[0] as u32)
            | (self.as_bytes()[1] as u32) << 8
            | (self.as_bytes()[2] as u32) << 16
            | (self.as_bytes()[3] as u32) << 24;
        let d2 = (self.as_bytes()[4] as u16) | (self.as_bytes()[5] as u16) << 8;
        let d3 = (self.as_bytes()[6] as u16) | (self.as_bytes()[7] as u16) << 8;
        let d4: &[u8; 8] = self.as_bytes()[8..16].try_into().unwrap();
        (d1, d2, d3, d4)
    }
    /// Returns a 128bit value containing the value.
    ///
    /// The bytes in the UUID will be packed directly into a `u128`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.as_u128(),
    ///     0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn as_u128(&self) -> u128 {
        u128::from_be_bytes(*self.as_bytes())
    }
    /// Returns a 128bit little-endian value containing the value.
    ///
    /// The bytes in the `u128` will be flipped to convert into big-endian
    /// order. This is based on the endianness of the UUID, rather than the
    /// target environment so bytes will be flipped on both big and little
    /// endian machines.
    ///
    /// Note that this will produce a different result than
    /// [`Uuid::to_fields_le`], because the entire UUID is reversed, rather
    /// than reversing the individual fields in-place.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.to_u128_le(),
    ///     0xd8d7d6d5d4d3d2d1c2c1b2b1a4a3a2a1,
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_u128_le(&self) -> u128 {
        u128::from_le_bytes(*self.as_bytes())
    }
    /// Returns two 64bit values containing the value.
    ///
    /// The bytes in the UUID will be split into two `u64`.
    /// The first u64 represents the 64 most significant bits,
    /// the second one represents the 64 least significant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    /// assert_eq!(
    ///     uuid.as_u64_pair(),
    ///     (0xa1a2a3a4b1b2c1c2, 0xd1d2d3d4d5d6d7d8),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn as_u64_pair(&self) -> (u64, u64) {
        let value = self.as_u128();
        ((value >> 64) as u64, value as u64)
    }
    /// Returns a slice of 16 octets containing the value.
    ///
    /// This method borrows the underlying byte value of the UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let bytes1 = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    /// let uuid1 = Uuid::from_bytes_ref(&bytes1);
    ///
    /// let bytes2 = uuid1.as_bytes();
    /// let uuid2 = Uuid::from_bytes_ref(bytes2);
    ///
    /// assert_eq!(uuid1, uuid2);
    ///
    /// assert!(std::ptr::eq(
    ///     uuid2 as *const Uuid as *const u8,
    ///     &bytes1 as *const [u8; 16] as *const u8,
    /// ));
    /// ```
    #[inline]
    pub const fn as_bytes(&self) -> &Bytes {
        &self.0
    }
    /// Consumes self and returns the underlying byte value of the UUID.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let bytes = [
    ///     0xa1, 0xa2, 0xa3, 0xa4,
    ///     0xb1, 0xb2,
    ///     0xc1, 0xc2,
    ///     0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8,
    /// ];
    /// let uuid = Uuid::from_bytes(bytes);
    /// assert_eq!(bytes, uuid.into_bytes());
    /// ```
    #[inline]
    pub const fn into_bytes(self) -> Bytes {
        self.0
    }
    /// Returns the bytes of the UUID in little-endian order.
    ///
    /// The bytes will be flipped to convert into little-endian order. This is
    /// based on the endianness of the UUID, rather than the target environment
    /// so bytes will be flipped on both big and little endian machines.
    ///
    /// # Examples
    ///
    /// ```
    /// use uuid::Uuid;
    ///
    /// # fn main() -> Result<(), uuid::Error> {
    /// let uuid = Uuid::parse_str("a1a2a3a4-b1b2-c1c2-d1d2-d3d4d5d6d7d8")?;
    ///
    /// assert_eq!(
    ///     uuid.to_bytes_le(),
    ///     ([
    ///         0xa4, 0xa3, 0xa2, 0xa1, 0xb2, 0xb1, 0xc2, 0xc1, 0xd1, 0xd2,
    ///         0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8
    ///     ])
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub const fn to_bytes_le(&self) -> Bytes {
        [
            self.0[3], self.0[2], self.0[1], self.0[0], self.0[5], self.0[4], self.0[7], self.0[6],
            self.0[8], self.0[9], self.0[10], self.0[11], self.0[12], self.0[13], self.0[14],
            self.0[15],
        ]
    }
    /// Tests if the UUID is nil (all zeros).
    pub const fn is_nil(&self) -> bool {
        self.as_u128() == u128::MIN
    }
    /// Tests if the UUID is max (all ones).
    pub const fn is_max(&self) -> bool {
        self.as_u128() == u128::MAX
    }
    /// A buffer that can be used for `encode_...` calls, that is
    /// guaranteed to be long enough for any of the format adapters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uuid::Uuid;
    /// let uuid = Uuid::nil();
    ///
    /// assert_eq!(
    ///     uuid.simple().encode_lower(&mut Uuid::encode_buffer()),
    ///     "00000000000000000000000000000000"
    /// );
    ///
    /// assert_eq!(
    ///     uuid.hyphenated()
    ///         .encode_lower(&mut Uuid::encode_buffer()),
    ///     "00000000-0000-0000-0000-000000000000"
    /// );
    ///
    /// assert_eq!(
    ///     uuid.urn().encode_lower(&mut Uuid::encode_buffer()),
    ///     "urn:uuid:00000000-0000-0000-0000-000000000000"
    /// );
    /// ```
    pub const fn encode_buffer() -> [u8; fmt::Urn::LENGTH] {
        [0; fmt::Urn::LENGTH]
    }
    /// If the UUID is the correct version (v1, v6, or v7) this will return
    /// the timestamp in a version-agnostic [`Timestamp`]. For other versions
    /// this will return `None`.
    ///
    /// # Roundtripping
    ///
    /// This method is unlikely to roundtrip a timestamp in a UUID due to the way
    /// UUIDs encode timestamps. The timestamp returned from this method will be truncated to
    /// 100ns precision for version 1 and 6 UUIDs, and to millisecond precision for version 7 UUIDs.
    pub const fn get_timestamp(&self) -> Option<Timestamp> {
        match self.get_version() {
            Some(Version::Mac) => {
                let (ticks, counter) = timestamp::decode_gregorian_timestamp(self);
                Some(Timestamp::from_gregorian(ticks, counter))
            }
            Some(Version::SortMac) => {
                let (ticks, counter) = timestamp::decode_sorted_gregorian_timestamp(self);
                Some(Timestamp::from_gregorian(ticks, counter))
            }
            Some(Version::SortRand) => {
                let millis = timestamp::decode_unix_timestamp_millis(self);
                let seconds = millis / 1000;
                let nanos = ((millis % 1000) * 1_000_000) as u32;
                Some(Timestamp::from_unix_time(seconds, nanos, 0, 0))
            }
            _ => None,
        }
    }
    /// If the UUID is the correct version (v1, or v6) this will return the
    /// node value as a 6-byte array. For other versions this will return `None`.
    pub const fn get_node_id(&self) -> Option<[u8; 6]> {
        match self.get_version() {
            Some(Version::Mac) | Some(Version::SortMac) => {
                let mut node_id = [0; 6];
                node_id[0] = self.0[10];
                node_id[1] = self.0[11];
                node_id[2] = self.0[12];
                node_id[3] = self.0[13];
                node_id[4] = self.0[14];
                node_id[5] = self.0[15];
                Some(node_id)
            }
            _ => None,
        }
    }
}

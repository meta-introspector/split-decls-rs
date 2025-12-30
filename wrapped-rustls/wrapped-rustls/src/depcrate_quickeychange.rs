// Generated macro for KeyChange (enum)
macro_rules! Depcrate_quicKeyChange {
() => {
// Module: crate::quic
// Provides: {"KeyChange"}
// Dependencies: {}
# [doc = " Key material for use in QUIC packet spaces"] # [doc = ""] # [doc = " QUIC uses 4 different sets of keys (and progressive key updates for long-running connections):"] # [doc = ""] # [doc = " * Initial: these can be created from [`Keys::initial()`]"] # [doc = " * 0-RTT keys: can be retrieved from [`ConnectionCommon::zero_rtt_keys()`]"] # [doc = " * Handshake: these are returned from [`ConnectionCommon::write_hs()`] after `ClientHello` and"] # [doc = "   `ServerHello` messages have been exchanged"] # [doc = " * 1-RTT keys: these are returned from [`ConnectionCommon::write_hs()`] after the handshake is done"] # [doc = ""] # [doc = " Once the 1-RTT keys have been exchanged, either side may initiate a key update. Progressive"] # [doc = " update keys can be obtained from the [`Secrets`] returned in [`KeyChange::OneRtt`]. Note that"] # [doc = " only packet keys are updated by key updates; header protection keys remain the same."] # [expect (clippy :: exhaustive_enums)] pub enum KeyChange { # [doc = " Keys for the handshake space"] Handshake { # [doc = " Header and packet keys for the handshake space"] keys : Keys , } , # [doc = " Keys for 1-RTT data"] OneRtt { # [doc = " Header and packet keys for 1-RTT data"] keys : Keys , # [doc = " Secrets to derive updated keys from"] next : Secrets , } , }
};
}

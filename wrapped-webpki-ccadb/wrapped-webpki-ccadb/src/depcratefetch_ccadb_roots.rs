// Generated macro for fetch_ccadb_roots (function)
macro_rules! Depcratefetch_ccadb_roots {
() => {
// Module: crate
// Provides: {"fetch_ccadb_roots"}
// Dependencies: {}
pub async fn fetch_ccadb_roots () -> BTreeMap < String , CertificateMetadata > { let root = include_bytes ! ("data/DigiCertGlobalRootCA.pem") ; let root = reqwest :: Certificate :: from_pem (root) . unwrap () ; let client = reqwest :: Client :: builder () . user_agent (format ! ("webpki-ccadb/v{}" , env ! ("CARGO_PKG_VERSION"))) . add_root_certificate (root) . build () . unwrap () ; let ccadb_url = "https://ccadb.my.salesforce-sites.com/mozilla/IncludedCACertificateReportPEMCSV" ; eprintln ! ("fetching {ccadb_url}...") ; let req = client . get (ccadb_url) . build () . unwrap () ; let csv_data = client . execute (req) . await . expect ("failed to fetch CSV") . text () . await . unwrap () ; let metadata = csv :: ReaderBuilder :: new () . has_headers (true) . from_reader (csv_data . as_bytes ()) . into_deserialize :: < CertificateMetadata > () . collect :: < Result < Vec < _ > , _ > > () . unwrap () ; let trusted_tls_roots = metadata . into_iter () . filter (CertificateMetadata :: trusted_for_tls) . collect :: < Vec < CertificateMetadata > > () ; let mut tls_roots_map = BTreeMap :: new () ; for root in trusted_tls_roots { match tls_roots_map . get (& root . sha256_fingerprint) { Some (_) => { panic ! ("duplicate fingerprint {}" , root . sha256_fingerprint) ; } None => { tls_roots_map . insert (root . sha256_fingerprint . clone () , root) ; } } } tls_roots_map }
};
}

// Generated macro for full_toml_parse (function)
macro_rules! Depcrate_configfull_toml_parse {
() => {
// Module: crate::config
// Provides: {"full_toml_parse"}
// Dependencies: {}
# [test] fn full_toml_parse () { const CONFIG_STR : & str = r#"
server_startup_timeout_ms = 10000

[dist]
# where to find the scheduler
scheduler_url = "http://1.2.3.4:10600"
# a set of prepackaged toolchains
toolchains = []
# the maximum size of the toolchain cache in bytes
toolchain_cache_size = 5368709120
cache_dir = "/home/user/.cache/sccache-dist-client"

[dist.auth]
type = "token"
token = "secrettoken"


#[cache.azure]
# does not work as it appears

[cache.disk]
dir = "/tmp/.cache/sccache"
size = 7516192768 # 7 GiBytes

[cache.gcs]
rw_mode = "READ_ONLY"
# rw_mode = "READ_WRITE"
cred_path = "/psst/secret/cred"
bucket = "bucket"
key_prefix = "prefix"
service_account = "example_service_account"

[cache.gha]
enabled = true
version = "sccache"

[cache.memcached]
# Deprecated alias for `endpoint`
# url = "127.0.0.1:11211"
endpoint = "tcp://127.0.0.1:11211"
# Username and password for authentication
username = "user"
password = "passwd"
expiration = 90000
key_prefix = "/custom/prefix/if/need"

[cache.redis]
url = "redis://user:passwd@1.2.3.4:6379/?db=1"
endpoint = "redis://127.0.0.1:6379"
cluster_endpoints = "tcp://10.0.0.1:6379,redis://10.0.0.2:6379"
username = "another_user"
password = "new_passwd"
db = 12
expiration = 86400
key_prefix = "/my/redis/cache"

[cache.s3]
bucket = "name"
region = "us-east-2"
endpoint = "s3-us-east-1.amazonaws.com"
use_ssl = true
key_prefix = "s3prefix"
no_credentials = true
server_side_encryption = false

[cache.webdav]
endpoint = "http://127.0.0.1:8080"
key_prefix = "webdavprefix"
username = "webdavusername"
password = "webdavpassword"
token = "webdavtoken"

[cache.oss]
bucket = "name"
endpoint = "oss-us-east-1.aliyuncs.com"
key_prefix = "ossprefix"
no_credentials = true
"# ; let file_config : FileConfig = toml :: from_str (CONFIG_STR) . expect ("Is valid toml.") ; assert_eq ! (file_config , FileConfig { cache : CacheConfigs { azure : None , disk : Some (DiskCacheConfig { dir : PathBuf :: from ("/tmp/.cache/sccache") , size : 7 * 1024 * 1024 * 1024 , preprocessor_cache_mode : PreprocessorCacheModeConfig :: activated () , rw_mode : CacheModeConfig :: ReadWrite , }) , gcs : Some (GCSCacheConfig { bucket : "bucket" . to_owned () , cred_path : Some ("/psst/secret/cred" . to_string ()) , service_account : Some ("example_service_account" . to_string ()) , rw_mode : CacheModeConfig :: ReadOnly , key_prefix : "prefix" . into () , credential_url : None , }) , gha : Some (GHACacheConfig { enabled : true , version : "sccache" . to_string () }) , redis : Some (RedisCacheConfig { url : Some ("redis://user:passwd@1.2.3.4:6379/?db=1" . to_owned ()) , endpoint : Some ("redis://127.0.0.1:6379" . to_owned ()) , cluster_endpoints : Some ("tcp://10.0.0.1:6379,redis://10.0.0.2:6379" . to_owned ()) , username : Some ("another_user" . to_owned ()) , password : Some ("new_passwd" . to_owned ()) , db : 12 , ttl : 24 * 3600 , key_prefix : "/my/redis/cache" . into () , }) , memcached : Some (MemcachedCacheConfig { url : "tcp://127.0.0.1:11211" . to_owned () , username : Some ("user" . to_owned ()) , password : Some ("passwd" . to_owned ()) , expiration : 25 * 3600 , key_prefix : "/custom/prefix/if/need" . into () , }) , s3 : Some (S3CacheConfig { bucket : "name" . to_owned () , region : Some ("us-east-2" . to_owned ()) , endpoint : Some ("s3-us-east-1.amazonaws.com" . to_owned ()) , use_ssl : Some (true) , key_prefix : "s3prefix" . into () , no_credentials : true , server_side_encryption : Some (false) , enable_virtual_host_style : None , }) , webdav : Some (WebdavCacheConfig { endpoint : "http://127.0.0.1:8080" . to_string () , key_prefix : "webdavprefix" . into () , username : Some ("webdavusername" . to_string ()) , password : Some ("webdavpassword" . to_string ()) , token : Some ("webdavtoken" . to_string ()) , }) , oss : Some (OSSCacheConfig { bucket : "name" . to_owned () , endpoint : Some ("oss-us-east-1.aliyuncs.com" . to_owned ()) , key_prefix : "ossprefix" . into () , no_credentials : true , }) , } , dist : DistConfig { auth : DistAuth :: Token { token : "secrettoken" . to_owned () } , # [cfg (any (feature = "dist-client" , feature = "dist-server"))] scheduler_url : Some (parse_http_url ("http://1.2.3.4:10600") . map (| url | { HTTPUrl :: from_url (url) }) . expect ("Scheduler url must be valid url str")) , # [cfg (not (any (feature = "dist-client" , feature = "dist-server")))] scheduler_url : Some ("http://1.2.3.4:10600" . to_owned ()) , cache_dir : PathBuf :: from ("/home/user/.cache/sccache-dist-client") , toolchains : vec ! [] , toolchain_cache_size : 5368709120 , rewrite_includes_only : false , } , server_startup_timeout_ms : Some (10000) , }) }
};
}

// Generated macro for REDIRECT_WITH_AUTH_JSON (const)
macro_rules! Depcrate_dist_client_authREDIRECT_WITH_AUTH_JSON {
() => {
// Module: crate::dist::client_auth
// Provides: {"REDIRECT_WITH_AUTH_JSON"}
// Dependencies: {}
const REDIRECT_WITH_AUTH_JSON : & str = r##"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"></head>
<body>
    <script>
    function writemsg(m) {
        document.body.appendChild(document.createTextNode(m.toString()));
        document.body.appendChild(document.createElement('br'));
    }
    function go() {
        writemsg('Retrieving details of authenticator...');
        fetch('/auth_detail.json').then(function (response) {
            if (!response.ok) {
                throw 'Error during retrieval - ' + response.status + ': ' + response.statusText;
            }
            writemsg('Using details to redirect to authentication page...');
            return response.json()
        }).then(function (auth_url) {
            window.location.href = auth_url;
        }).catch(writemsg);
    }
    go();
    </script>
</body>
</html>
"## ;
};
}

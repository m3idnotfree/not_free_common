use std::path::PathBuf;

use not_free_common::Oauth2;

#[test]
fn oauth2_base() {
    let oauth2 = Oauth2::new(
        "target",
        "client_id",
        "client_secret",
        5000,
        "state",
        PathBuf::from("/path/to/test.json"),
        "redirect_url",
    );

    let expect_auth_request_url =
        url::Url::parse("target?client_id=client_id&redirect_uri=redirect_url&response_type=code&scope=chat:read&state=state");

    assert_eq!(
        oauth2.auth_request_url("chat:read"),
        expect_auth_request_url
    );
}

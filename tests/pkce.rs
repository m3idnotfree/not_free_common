use not_free_common::pkce::{Pkce, PkceError};
use ring::error::Unspecified;

#[test]
fn pkcg_base() {
    let (pkce_challenge, pkce_verify) = Pkce::new_sha256().unwrap();
    assert_eq!(Ok(()), pkce_verify(pkce_challenge));
}
#[test]
fn pkcg_base2() {
    let (_, pkce_verify) = Pkce::new_sha256().unwrap();
    let (pkce_challenge2, _) = Pkce::new_sha256().unwrap();
    assert_eq!(
        Err(PkceError::VerifyHmac(Unspecified)),
        pkce_verify(pkce_challenge2)
    );
}

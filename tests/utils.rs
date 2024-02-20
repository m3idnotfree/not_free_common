use not_free_common::query_find;
use url::Url;

#[test]
fn query_find_base() {
    let url = Url::parse("https://example.com/products?page=2").unwrap();
    assert_eq!(query_find(&url, "page"), "2");
}

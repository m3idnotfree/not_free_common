pub fn query_find(url: &url::Url, key: &str) -> String {
    let result_pair = url
        .query_pairs()
        .find(|pair| {
            let (k, _) = pair;
            k == key
        })
        .unwrap();

    let (_, value) = result_pair;
    value.into_owned()
}

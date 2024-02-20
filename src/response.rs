pub fn query_find(url: &url::Url, key: &str) -> String {
    let code_pair = url
        .query_pairs()
        .find(|pair| {
            // let &(ref k, _) = pair;
            let (k, _) = pair;
            k == key
        })
        .unwrap();

    let (_, value) = code_pair;
    value.into_owned()
}

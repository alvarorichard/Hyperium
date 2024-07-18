pub fn get_client() -> reqwest::Client {
    reqwest::Client::builder()
        .brotli(true)
        .cookie_store(true)
        .build()
        .unwrap()
}

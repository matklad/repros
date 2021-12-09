fn main() {
    awc::Client::builder()
        .connector(awc::Connector::new().max_http_version(awc::http::Version::HTTP_11))
        .finish();
}

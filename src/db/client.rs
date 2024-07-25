use aws_sdk_dynamodb::Client;

pub async fn create_local_client() -> Client {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .endpoint_url("http://localhost:8000")
        .region("local")
        .load()
        .await;
    Client::new(&config)
}

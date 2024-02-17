use tikv_client::RawClient;
#[tokio::main]
async fn main() {
    // println!("hii");
    let client = RawClient::new(vec!["127.0.0.1:2379"]).await.unwrap();
    client.put("key".to_owned(), "value".to_owned()).await.unwrap();
    let value = client.get("key".to_owned()).await.unwrap();
    println!("{:?}",value);
}

use iota_client::Client;
use iota_client::Result;

pub async fn node_info_dev() -> Result<()>
{
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);

    Ok(())
}

pub async fn node_info_main() -> Result<()>
{
    let iota = Client::builder()
        .with_node("https://chrysalis-nodes.iota.org")
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);

    Ok(())
}
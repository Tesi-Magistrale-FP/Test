use iota_client::Client;
use iota_client::Result;

// Funzione per ottenere le informazioni sul nodo che svolge il ruolo di load balancer per una specifica rete
pub async fn get_node_info(net_url: String) -> Result<()> 
{
    let iota = Client::builder()
        .with_node(&net_url)
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);

    Ok(())
}
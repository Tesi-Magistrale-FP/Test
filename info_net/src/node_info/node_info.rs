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
    println!("- Nome: {}\n- Versione: {}\n- Attivo: {}\n- Rete: {}\n- Url: {}\n- Min. PoW score: {}\n- Messaggi ricevuti al secondo: {}\n- Messaggi referenziati al secondo: {}\n- Features: {}, {}", info.nodeinfo.name, info.nodeinfo.version, info.nodeinfo.is_healthy, info.nodeinfo.network_id, info.url, info.nodeinfo.min_pow_score, info.nodeinfo.messages_per_second, info.nodeinfo.referenced_messages_per_second, info.nodeinfo.features[0], info.nodeinfo.features[1]);

    Ok(())
}
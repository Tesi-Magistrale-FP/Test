use std::time::Instant;
use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Subscriber, MessageContent},
    core::{println, Result},
};

use crate::utility::scrittura_file::scrivi_file;

// Funzione per misurare il tempo necessario per leggere le transazioni Streams attaccate precedentemente al Tangle su una rete specifica
#[allow(unused_must_use)]
pub async fn test_3(sent_msgs: &Vec<String>, subscriber: Subscriber<Client>, path_ris: String) -> Result<()> 
{
    let mut subscriber = subscriber;

    let inizio = Instant::now();                                                                      	// Inizio misurazione tempo

    let retrieved_msgs = subscriber.fetch_all_next_msgs().await;        								// Lettura di tutti i messaggi pubblicati sul canale
    
    let tempo_lettura = format!("{}", inizio.elapsed().as_millis());                                   	// Fine misurazione tempo
   
    println!("- Fine lettura messaggi dal Tangle");

    let processed_msgs = retrieved_msgs                    												// Iterazione su tutti i messaggi letti per assicurarsi che matchino con quelli inviati
        .iter()
        .map(|msg| {
            let content = &msg.body;
            match content {
                MessageContent::SignedPacket {
                    pk: _,
                    public_payload: _,
                    masked_payload,
                } => String::from_utf8(masked_payload.0.to_vec()).unwrap(),
                _ => String::default(),
            }
        })
        .filter(|s| s != &String::default())
        .collect::<Vec<String>>();

    if processed_msgs.is_empty() && sent_msgs.is_empty()                                              	// Errori durante il matching
    {
        return Ok(());
    }

    scrivi_file(path_ris.clone(), vec![tempo_lettura]);                                                	// Scrittura dei tempi misurati sul Tangle
    println!("- Fine scrittura tempi nel file {}", path_ris.clone());

    Ok(())
}
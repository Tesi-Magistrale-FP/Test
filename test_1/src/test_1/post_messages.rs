use iota_client::Client;
use iota_client::Result;
use std::time::Instant;

use crate::utility::scrittura_file::scrivi_file;

// Funzione per misurare il tempo necessario per creare e attaccare delle transazioni semplici al Tangle su una rete specifica
#[allow(unused_must_use)]
pub async fn test_1(coordinate: Vec<String>, node_url: String, path_ris: String) -> Result<()>
{
    let iota = Client::builder()                                                      					// Ottengo i riferimenti del nodo della rete che dovrà attaccare i messaggi al Tangle
        .with_node(&node_url)?
        .finish()
        .await?;

    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi per ogni transazione
    let mut contatore = 0;                                                                             	// Contatore che mi indica il numero di transazioni processate
    let mut iteratore = coordinate.iter();
    while let Some(valore) = iteratore.next()                                                      		// Per ogni transazione
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        iota.message().with_index("TrackApp").with_data(valore.as_bytes().to_vec()).finish().await?;  	// Creo il messaggio con l'index TrackApp, il valore delle coordinate e lo attacco al Tangle

        tempi.push(format!("{}", inizio.elapsed().as_millis()));                                       	// Fine misurazione tempo
    
        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 transazioni attaccate al Tangle
        {
            println!("--- {} messaggi attaccati al Tangle", contatore);                                 // Mostro un messaggio
        }
    }
    println!("- Fine attaccamento messaggi al Tangle");

    scrivi_file(path_ris.clone(), tempi);                                                           	// Scrittura dei tempi misurati sul Tangle
    println!("- Fine scrittura tempi nel file {}", path_ris.clone());

    Ok(())
}
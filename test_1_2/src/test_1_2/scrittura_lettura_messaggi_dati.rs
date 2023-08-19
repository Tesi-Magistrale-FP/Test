use iota_client::Client;
use iota_client::Result;
use std::time::Instant;

use crate::utility::scrittura_file::scrivi_file;

// Funzione per effettuare il test 1 e 2
// Il test 1 consiste nella misurazione dei tempi necessari per creare e scrivere nel Tangle un messaggio contenente dei dati
// Il test 2 consiste nella lettura dei tempi necessari per leggere dal Tangle e ottenere il contenuto di un messaggio dati precedentemente scritto nel test 1
#[allow(unused_must_use)]
pub async fn test_1_2(coordinate: Vec<String>, node_url: String, path_ris_s: String, path_ris_l: String) -> Result<()>
{
    let iota = Client::builder()                                                      					// Ottengo i riferimenti del nodo della rete che dovrà attaccare e leggere i messaggi sul Tangle
        .with_node(&node_url)?
        .finish()
        .await?;

//////
	// TEST 1 - SCRITTURA MESSAGGI DATI
//////

    let mut id_msg = vec![];                                                                            // Vettore che conterrà gli ID dei messaggi scritti sul Tangle
    let mut tempi_s = vec![];                                                                    		// Vettore che conterrà i tempi di scrittura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore_s = coordinate.iter();
    while let Some(valore) = iteratore_s.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let messaggio = iota.message().with_index("TrackApp").with_data(valore.as_bytes().to_vec()).finish().await?;  	// Creo il messaggio con l'index TrackApp, il valore delle coordinate e lo attacco al Tangle

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        tempi_s.push(format!("{}", fine));                                       						// Salvataggio tempo registrato

        id_msg.push(messaggio.id().0);																	// Ottengo l'ID del messaggio appena scritto e lo salvo per il test 2

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 transazioni attaccate al Tangle
        {
            println!("--- {} messaggi attaccati al Tangle", contatore);                                 // Mostro un messaggio
        }
    }
    println!("- Fine attaccamento messaggi al Tangle");

    scrivi_file(path_ris_s.clone(), tempi_s);                                                           // Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris_l.clone());

//////
	// TEST 2 - LETTURA MESSAGGI DATI
//////

    println!("\n---------------------------------------");
    if node_url == String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/")
    {
        println!("\nTest 2 - Lettura delle transazioni semplici appena scritte sul Tangle - Devnet\n");
    }
    else if node_url == String::from("https://chrysalis-nodes.iota.org")
    {
        println!("\nTest 2 - Lettura delle transazioni semplici appena scritte sul Tangle - Mainnet\n");
    }

    let mut tempi_l = vec![];                                                                    		// Vettore che conterrà i tempi di lettura per ogni messaggio
    contatore = 0;                                                                             			// Resetto il contatore che indica il numero di messaggi processati
    let mut iteratore_l = id_msg.iter();
    while let Some(valore) = iteratore_l.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let _dato = iota.get_message().metadata(&valore).await?;										// Leggo il messaggio dal Tangle usando il suo ID e prendo il suo contenuto

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        tempi_l.push(format!("{}", fine));                                       						// Salvataggio tempo registrato

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 transazioni attaccate al Tangle
        {
            println!("--- {} messaggi letti dal Tangle", contatore);                                 	// Mostro un messaggio
        }
    }
    println!("- Fine lettura messaggi dal Tangle");

    scrivi_file(path_ris_l.clone(), tempi_l);                                                           // Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris_l.clone());

    Ok(())
}
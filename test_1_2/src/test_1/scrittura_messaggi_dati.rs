use iota_client::{ Client, Result };
use std::time::Instant;

use crate::utility::scrittura_file::scrivi_file;
use crate::test_2::lettura_messaggi_dati::test_2;

// Funzione per effettuare il test 1, che consiste nella misurazione del tempo necessario per creare e scrivere nel Tangle un messaggio contenente dei dati
#[allow(unused_must_use)]
pub async fn test_1(coordinate: Vec<String>, node_url: String, path_ris_s: String, path_ris_l: String) -> Result<()>
{
    let iota = Client::builder()                                                      					// Ottengo i riferimenti del nodo della rete che dovrà scrivere e leggere i messaggi sul Tangle
        .with_node(&node_url)?
        .finish()
        .await?;

//////
	// TEST 1 - SCRITTURA MESSAGGI DATI
//////

    let mut id_msg = vec![];                                                                            // Vettore che conterrà gli ID dei messaggi scritti sul Tangle
    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di scrittura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore_s = coordinate.iter();
    while let Some(valore) = iteratore_s.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let messaggio = iota.message().with_index("TrackApp").with_data(valore.as_bytes().to_vec()).finish().await?;  	// Creo il messaggio con l'index TrackApp, il valore delle coordinate e lo attacco al Tangle

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        id_msg.push(messaggio.id().0);																	// Ottengo l'ID del messaggio appena scritto e lo salvo per il test 2

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 messaggi attaccati al Tangle
        {
            println!("--- {} messaggi attaccati al Tangle", contatore);                                 // Mostro un messaggio
        }
    }
    println!("- Fine attaccamento messaggi al Tangle");

    scrivi_file(path_ris_s.clone(), tempi);																// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris_s.clone());

//////
	// TEST 2 - LETTURA MESSAGGI DATI
//////

    println!("\n---------------------------------------");
    if node_url == String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/")
    {
        println!("\nTest 2 - Lettura dei messaggi dati appena scritti sul Tangle - Devnet\n");
    }
    else if node_url == String::from("https://chrysalis-nodes.iota.org")
    {
        println!("\nTest 2 - Lettura dei messaggi dati appena scritti sul Tangle - Mainnet\n");
    }
    test_2(iota, id_msg, path_ris_l).await?;															// Eseguo il test 2

    Ok(())
}
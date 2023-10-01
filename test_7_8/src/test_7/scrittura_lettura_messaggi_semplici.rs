use std::time::Instant;
use anyhow::Result;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use futures::stream::TryStreamExt;
use serde::{Deserialize, Serialize};

use crate::utility::scrittura_file::scrivi_file;

#[derive(Debug, Serialize, Deserialize)]
struct Valore {																							// Struttura usata per organizzare i dati da archiviare sul database
    id: String,
    valore: String,
}

// Funzione per effettuare il test 7, che consiste nella misurazione del tempo necessario per scrivere e leggere dei messaggi semplici su database
#[allow(unused_must_use)]
pub async fn test_7(coordinate: Vec<String>, path_ris_s: String, path_ris_l: String) -> Result<()>
{
    // Inizializzazione client che si occuperà della comunicazione con MongoDB
    let mut client_options = ClientOptions::parse("mongodb+srv://admin:Mongo160498FP!*@cluster.8kqboxd.mongodb.net/?retryWrites=true&w=majority").await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    let db = client.database("Rust-DB");
    let collection = db.collection::<Valore>("testSemplice");

    // SCRITTURA DEI MESSAGGI SUL DATABASE

    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di scrittura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore_s = coordinate.iter();
    while let Some(valore) = iteratore_s.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo
        
        let valore_struct: Valore = Valore { id: contatore.to_string(), valore: valore.to_string() };				// Crea la struttura che conterrà i valori da salvare sul database
        collection.insert_one(valore_struct, None).await?;                                   			// Scrittura messaggio sul database      

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                
        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 messaggi scritti sul database
        {
            println!("--- {} messaggi memorizzati sul database", contatore);                            // Mostro un messaggio
        }
    }
    println!("- Fine memorizzazione messaggi sul database");

    scrivi_file(path_ris_s.clone(), tempi);																// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi di scrittura nel file {}\n|", path_ris_s.clone());

    // LETTURA DEI MESSAGGI DAL DATABASE

    tempi = vec![];                                                                    					// Reset del vettore che conterrà i tempi di scrittura per ogni messaggio
    for id in 0..coordinate.len() as i32																// Per ogni messaggio scritto sul database
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let filter = doc! { "id": id.to_string() };														// Imposto un filtro di ricerca basato sul campo "id"

        let mut cursor = collection.find(filter, None).await?;											// Ricerca di tutti i messaggi che rispettano il filtro impostato

        let valore_doc: Valore = cursor.try_next().await?.unwrap(); 									// Ottengo il valore del valore letto dal database
        let _valore: String = valore_doc.valore;

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                
        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        if((id+1) % 10) == 0                                                                         	// Ogni 10 messaggi sletti dal database
        {
            println!("--- {} messaggi letti dal database", (id+1));                            			// Mostro un messaggio
        }
    }
    println!("- Fine lettura messaggi dal database");

    scrivi_file(path_ris_l.clone(), tempi);																// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi di lettura nel file {}", path_ris_l.clone());

    Ok(())
}
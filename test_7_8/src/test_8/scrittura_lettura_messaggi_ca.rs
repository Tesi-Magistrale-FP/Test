use std::{time::Instant, str::FromStr};
use anyhow::Result;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};
use futures::stream::TryStreamExt;
use aes_gcm::{aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm};
use serde::{Deserialize, Serialize};
use ed25519_dalek::{SigningKey, Signature, Signer};

use crate::utility::scrittura_file::scrivi_file;

#[derive(Debug, Serialize, Deserialize)]
struct Valore {																							// Struttura usata per organizzare i dati da archiviare sul database
    id: String,
    valore: Vec<u8>,
}

// Funzione per effettuare il test 8, che consiste nella misurazione del tempo necessario per scrivere e leggere dei messaggi cifrati e autenticati su database
#[allow(unused_must_use)]
pub async fn test_8(coordinate: Vec<String>, path_ris_s: String, path_ris_l: String) -> Result<()>
{
    // Inizializzazione client che si occuperà della comunicazione con MongoDB
    let mut client_options = ClientOptions::parse("mongodb+srv://admin:Mongo160498FP!*@cluster.8kqboxd.mongodb.net/?retryWrites=true&w=majority").await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    let db = client.database("Rust-DB");
    let collection = db.collection::<Valore>("testCifratoAutenticato");

    // Inizializzazione crifrario simmetrico AES-GCM per cifratura simmetrica
    let key = Aes256Gcm::generate_key(OsRng);															// Generazione chiave simmetrica
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // Inizializzazione cifrario asimmetrico Ed25519 per firma digitale
    let mut csprng = OsRng;
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);                                   	// Creazione coppia di chiavi: privata per firmare e pubblica per verificare la firma digitale

    // SCRITTURA DEI MESSAGGI SUL DATABASE

    let mut valori: Vec<String> = vec![];                                                               // Vettore che conterrà tutti i valori salvati nel database e che serviranno per la verifica della firma digitale
    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di scrittura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore_s = coordinate.iter();
    while let Some(valore) = iteratore_s.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo
        
        let firma_digitale: Signature = signing_key.sign(valore.as_bytes().as_ref());                   // Firma digitale del valore con la chiave privata
        let firma_digitale_s: String = firma_digitale.to_string();
        let testo_cifrato: Vec<u8> = cipher.encrypt(&nonce, firma_digitale_s.as_bytes().as_ref()).unwrap();		// Cifratura messaggi con chiave simmetrica		
        let valore_struct: Valore = Valore { id: contatore.to_string(), valore: testo_cifrato };				// Crea la struttura che conterrà i valori da salvare sul database
        collection.insert_one(valore_struct, None).await?;                                   			// Scrittura messaggio sul database      

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                
        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato
        
        valori.push(valore.to_string());

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

        let valore_doc: Valore = cursor.try_next().await?.unwrap(); 									// Ottengo dato letto dal database
        let testo_cifrato: Vec<u8> = valore_doc.valore;													// Ottengo il valore del dato letto
        let testo_decifrato: Vec<u8> = cipher.decrypt(&nonce, testo_cifrato.as_ref()).unwrap();			// Decifro il valore letto
        
        let firma_digitale_s: String = unsafe { String::from_utf8_unchecked(testo_decifrato) };         // Ottengo la firma digitale
        let firma_digitale: Signature = Signature::from_str(&firma_digitale_s).unwrap();
        let verifica_firma: bool = signing_key.verify(&valori[id as usize].as_bytes().to_vec(), &firma_digitale).is_ok();

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
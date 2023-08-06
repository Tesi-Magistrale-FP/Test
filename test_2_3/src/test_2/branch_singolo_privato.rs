use rand::Rng;
use core::str::FromStr;
use std::time::Instant;
use iota_streams::{
    app::transport::tangle::client::Client,
    app_channels::api::tangle::{Address, Author, Bytes, ChannelType, Subscriber},
    core::{println, Result},
};

use crate::utility::scrittura_file::scrivi_file;
use crate::test_3::lettura_messaggi::test_3;

const ALPH9: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9";

// Funzione per misurare il tempo necessario per creare e attaccare delle transazioni Streams al Tangle su una rete specifica
#[allow(unused_must_use)]
pub async fn test_2(coordinate: Vec<String>, node_url: String, path_ris: String) -> Result<()> 
{
    // Lato Author -> Creazione dell'Author e del canale 

    let seed: &str = &(0..81)																			// Generazione di un seed univoco per l'Author                                                                       
		.map(|_| {
            ALPH9
                .chars()
                .nth(rand::thread_rng().gen_range(0..27))
                .unwrap()
        })
        .collect::<String>();

    let client = Client::new_from_url(&node_url);                                                      	// Creazione di un Transport Client

    let mut author = Author::new(seed, ChannelType::SingleBranch, client.clone());          			// Generazione dell'Author

    let announcement_link = author.send_announce().await?;                                             	// Creazione di un canale con il relativo messaggio di annuncio
    let ann_link_string = announcement_link.to_string();                                               	// Ottenimento del link che fungerà come root (punto di accesso) del canale
    
    // ----------------------------------------------------------------------
    // Lato Subscriber -> Creazione del Subscriber e relativa iscrizione al canale

    let mut subscriber = Subscriber::new("Subscriber", client.clone());             					// Generazione del Subscriber

    let ann_address = Address::from_str(&ann_link_string)?;                                            	// Generazione di un oggetto Address per fornire il messaggio di annuncio dell'Author
    subscriber.receive_announcement(&ann_address).await?;                                              	// Ricezione del messaggio di annuncio per iniziare ad ascoltare sul canale

    let subscribe_msg = subscriber.send_subscribe(&ann_address).await?;                         		// Invio del messaggio di iscrizione collegato con il messaggio di annuncio

    let sub_msg_str = subscribe_msg.to_string();                                                       	// Ottenimento dei link di iscrizione che saranno usati dall'Author per completare l'iscrizione del Subscriber

    // ----------------------------------------------------------------------
    // Lato Author -> Conferma iscrizione del Subscriber al canale e pubblicazione dei messaggi

    let sub_address = Address::from_str(&sub_msg_str)?;                                                     
    author.receive_subscribe(&sub_address).await?;                                                     	// L'Author procede alla conferma dell'iscrizione del Subscriber

    // Ora Subscriber è correttamente iscritto al canale
    // Author invia keyload con il Subscriber linkato nel messaggio di annuncio
    // Questo restituisce una tupla contenente i link ai messaggi (linkati al messaggio di annuncio)
    let (keyload_link, _seq) = author.send_keyload_for_everyone(&announcement_link).await?;

    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi per ogni transazione
    let mut contatore = 0;                                                                             	// Contatore che mi indica il numero di transazioni processate
    let mut prev_msg_link = keyload_link;                                                               // Link al messaggio precedente
    let mut iteratore = coordinate.iter();
    while let Some(valore) = iteratore.next()                                                      		// Per ogni transazione
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let (msg_link, _seq_link) = author.send_signed_packet(          								// Creo il messaggio Streams cifrato e firmato dall'Author e lo attacco al tangle
            &prev_msg_link,
            &Bytes::default(),
            &Bytes(valore.as_bytes().to_vec()),
        ).await?;
        prev_msg_link = msg_link;                                                                       // Aggiorno il link al messaggio precedente

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

    // -----------------------------------------------------------------------------
    // Lato Subscriber -> Subscriber legge i messaggi appena pubblicati sul canale
    
    println!("\n---------------------------------------");
    if node_url == String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/")
    {
        println!("\nTest 3 - Lettura delle {} transazioni Streams appena pubblicate sul Tangle - Devnet\n", coordinate.len().to_string());
        test_3(&coordinate, subscriber, String::from("./risultati/risultato_t3_devnet.csv")).await?;
    }
    else if node_url == String::from("https://chrysalis-nodes.iota.org")
    {
        println!("\nTest 3 - Lettura delle {} transazioni Streams appena pubblicate sul Tangle - Mainnet\n", coordinate.len().to_string());
        test_3(&coordinate, subscriber, String::from("./risultati/risultato_t3_mainnet.csv")).await?;
    }

    Ok(())
}
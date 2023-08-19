use iota_client:: { Client, Result, bee_message::MessageId };
use std::time::Instant;

use crate::utility::scrittura_file::scrivi_file;

// Funzione per effettuare il test 2, che consiste nella misurazione del tempo necessario per leggere dal Tangle un messaggio, scritto nel test 1, e accedere ai suoi dati 
#[allow(unused_must_use)]
pub async fn test_2(iota: Client, id_msg: Vec<MessageId>, path_ris: String) -> Result<()>
{
    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di lettura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore = id_msg.iter();
    while let Some(valore) = iteratore.next()                                                      		// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        let _dato = iota.get_message().metadata(&valore).await?;										// Leggo il messaggio dal Tangle usando il suo ID e ottengo il suo contenuto

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 messaggi attaccati al Tangle
        {
            println!("--- {} messaggi letti dal Tangle", contatore);                                 	// Mostro un messaggio
        }
    }
    println!("- Fine lettura messaggi dal Tangle");

    scrivi_file(path_ris.clone(), tempi);                                                           	// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris.clone());

    Ok(())
}
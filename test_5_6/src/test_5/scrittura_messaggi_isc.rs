use std::process::Command;
use std::time::Instant;
use anyhow::Result;

use crate::utility::scrittura_file::scrivi_file;

// Funzione per effettuare il test 5, che consiste nella misurazione del tempo necessario per scrivere dei messaggi sullo stato interno di un ISC
#[allow(unused_must_use)]
pub async fn test_5(coordinate: Vec<String>, path_ris_s: String) -> Result<()>
{
    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di scrittura per ogni messaggio
    let mut contatore = 0;                                                                             	// Contatore che indica il numero di messaggi processati
    let mut iteratore_s = coordinate.iter();
    while let Some(valore) = iteratore_s.next()                                                      	// Per ogni messaggio
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        // Esegue il comando della wasp-cli per eseguire la memorizzazione dei messaggi sullo ISC Performance
        let mut output = Command::new("wasp-cli")
        .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
        .args(["chain", "post-request", "performance", "memorizzaValore", "String", "valore", "String", &valore, "--chain=mychain", "-s"])
        .output()
        .unwrap();

        // Ottiene l'output del comando eseguito e lo elabora per capire l'esito della memorizzazione
        let mut output_s: String = String::from_utf8(output.stdout).unwrap();

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        if ! output_s.contains("Waiting for tx requests to be processed...")                            // Errore esecuzione comando per chiamare la funzione 
        {
            println!("- {contatore} | Errore memorizzazione valore -> {output_s}");
        }
        else                                                                                      		// Chiamata alla funzione avvenuta con successo
        {
            // Recupera l'indirizzo della transazione usata per chiamare la funzione
            let  output_elab: String = output_s.replace("Waiting for tx requests to be processed...", "");
            let ind_trans: String = String::from(output_elab.split_whitespace().last().unwrap().replace(")", ""));

            // Recupera l'evento generato dalla funzione per comprendere l'esito della memorizzazione
            output = Command::new("wasp-cli")
                .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
                .args(["chain", "request", &ind_trans])
                .output()
                .unwrap();

            output_s = String::from_utf8(output.stdout).unwrap();

            if ! output_s.contains("performance.valoreRegistrato")                                    	// Memorizzazione avvenuta con successo
            {
                println!("- {contatore} non memorizzato!");
            }
        }

        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        contatore += 1;                                                                                 // Incremento il contatore
        if(contatore % 10) == 0                                                                         // Ogni 10 messaggi scritti sullo ISC
        {
            println!("--- {} messaggi memorizzati sullo ISC", contatore);                              	// Mostro un messaggio
        }
    }
    println!("- Fine memorizzazione messaggi sullo ISC");

    scrivi_file(path_ris_s.clone(), tempi);																// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris_s.clone());

    Ok(())
}
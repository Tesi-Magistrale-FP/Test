use std::process::{Command, Stdio};
use std::time::Instant;
use anyhow::Result;

use crate::utility::scrittura_file::scrivi_file;

// Funzione per effettuare il test 6, che consiste nella misurazione del tempo necessario per leggere dei messaggi dallo stato interno di un ISC
#[allow(unused_must_use)]
pub async fn test_6(numero_messaggi: i32, path_ris_l: String) -> Result<()>
{
    let mut tempi = vec![];                                                                    			// Vettore che conterrà i tempi di scrittura per ogni messaggio
    for contatore in 0..numero_messaggi
    {
        let inizio = Instant::now();                                                               		// Inizio misurazione tempo

        // Esegue il comando della wasp-cli per eseguire la lettura dei valori dallo ISC Performance
        let cmd_view = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["chain", "call-view", "performance", "ottieniValore", "String", "indice", "Int32", &contatore.to_string(), "--chain=mychain"])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let cmd_res = Command::new("wasp-cli")
            .current_dir("C:\\Users\\fra-p\\Desktop\\Nodo_Wasp\\tools\\local-setup")
            .args(["decode", "string", "valore", "string"])
            .stdin(Stdio::from(cmd_view.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let output_view = cmd_res.wait_with_output().unwrap();

        // Ottiene l'output del comando eseguito e lo elabora per ottenere il valore letto
        let output_s: String = String::from_utf8(output_view.stdout).unwrap().replace("valore: ", "").replace("\"", "").replace("\n", "");

        let fine = inizio.elapsed().as_millis();                            							// Fine misurazione tempo                

        if output_s.len() == 0 																			// Nessun valore letto
        {
            println!("- {contatore} nessun valore letto!");
        }

        tempi.push(format!("{}", fine));                                       							// Salvataggio tempo registrato

        if((contatore+1) % 10) == 0                                                                     // Ogni 10 messaggi scritti sullo ISC
        {
            println!("--- {} messaggi letti dallo ISC", (contatore+1));                              	// Mostro un messaggio
        }
    }
    println!("- Fine lettura messaggi dallo ISC");

    scrivi_file(path_ris_l.clone(), tempi);																// Scrittura dei tempi misurati nell'apposito file
    println!("- Fine scrittura tempi nel file {}", path_ris_l.clone());

    Ok(())
}
use std::error::Error;
use csv::Writer;

// Funzione per scrivere i valori passati all'interno di un file csv
pub fn scrivi_file(path: String, valori: Vec<String>) -> Result<(), Box<dyn Error>> 
{
    let mut wtr = Writer::from_path(path)?;                                 							// Apertura file csv. Lo crea se non esiste. Lo apre in sovrascrittura.

    let mut iteratore = valori.iter();
    let mut transazione = 1;
    while let Some(valore) = iteratore.next()                                    						// Per ogni valore da scrivere
    {
        wtr.write_record(&[transazione.to_string(), String::from(valore)])?;              				// Scrivo in una nuova riga "ID transazione, valore"

        transazione += 1;
    }
    wtr.flush()?;

    Ok(())
}
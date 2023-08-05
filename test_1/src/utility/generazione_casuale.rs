use rand::Rng;

// Funzione per generare casualmente "max_esc" coordinate (latitudine, longitudine) e restituirle in un vettore di stringhe
pub fn genera_coordinate(max_esc: i32) -> Vec<String>                                             
{
    let mut coordinate = vec![];                                            // Vettore che conterrà le coordinate (latitudine, longitudine) generate casualmente 
    for _i in 0..max_esc                                                            // Genero casualmente i valori delle coordinate dell'utente (latitudine, longitudine) per ogni transazione
    {
        let lat = rand::thread_rng().gen_range(-90.0..=90.0);                       // Generazione casuale dalla latitudine
        let lon = rand::thread_rng().gen_range(-180.0..=180.0);                     // Generazione casuale della longitudine

        coordinate.push(format!("({}, {})", lat, lon));                                  // Inserimento delle coordinate generate nel vettore
    }

    return coordinate;                                                                   // Restituisco il vettore con tutte le coordinate generate casualmente
}
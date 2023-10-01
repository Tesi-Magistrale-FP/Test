use anyhow::Result;

mod test_7;
mod test_8;
mod utility;

const NUM_MESSAGGI: i32 = 100;

#[tokio::main]
async fn main() -> Result<()> 
{
    println!("\nTest 7-8 - Generazione casuale delle coordinate\n");
    let coordinate: Vec<String> = utility::generazione_casuale::genera_coordinate(NUM_MESSAGGI);
    println!("- {} coordinate generate con successo", NUM_MESSAGGI);
    
    println!("\n---------------------------------------");
    println!("\nTest 7 - Scrittura e lettura di {} messaggi semplici su database\n", NUM_MESSAGGI);
    test_7::scrittura_lettura_messaggi_semplici::test_7(coordinate.clone(), String::from("./risultati/risultato_t7_scrittura.csv"), String::from("./risultati/risultato_t7_lettura.csv")).await?;
    
    println!("\n---------------------------------------");
    println!("\nTest 8 - Scrittura e lettura di {} messaggi cifrati e autenticati su database\n", NUM_MESSAGGI);
    test_8::scrittura_lettura_messaggi_ca::test_8(coordinate.clone(), String::from("./risultati/risultato_t8_scrittura.csv"), String::from("./risultati/risultato_t8_lettura.csv")).await?;

    println!();

    Ok(())
}
use anyhow::Result;

mod test_5;
mod test_6;
mod utility;

const NUM_MESSAGGI: i32 = 100;

#[tokio::main]
async fn main() -> Result<()>
{
    println!("\nTest 5-6 - Generazione casuale delle coordinate\n");
    let coordinate: Vec<String> = utility::generazione_casuale::genera_coordinate(NUM_MESSAGGI);
    println!("- {} coordinate generate con successo", NUM_MESSAGGI);

    println!("\n---------------------------------------");
    println!("\nTest 5 - Scrittura di {} messaggi sullo IOTA Smart Contracts\n", NUM_MESSAGGI);
    test_5::scrittura_messaggi_isc::test_5(coordinate.clone(), String::from("./risultati/risultato_t5_pc.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nTest 6 - Lettura di {} messaggi dallo IOTA Smart Contracts\n", NUM_MESSAGGI);
    test_6::lettura_messaggi_isc::test_6(NUM_MESSAGGI, String::from("./risultati/risultato_t6_pc.csv")).await?;

    println!();
    Ok(())
}
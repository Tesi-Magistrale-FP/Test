use anyhow::Result;

mod test_3;
mod test_4;
mod utility;

const NUM_MESSAGGI: i32 = 100;

#[tokio::main]
async fn main() -> Result<()> 
{
    let url_devnet: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet = String::from("https://chrysalis-nodes.iota.org");

    println!("\nTest 3-4 - Generazione casuale delle coordinate\n");
    let coordinate = utility::generazione_casuale::genera_coordinate(NUM_MESSAGGI);
    println!("- {} coordinate generate con successo", NUM_MESSAGGI);

    println!("\n---------------------------------------");
    println!("\nTest 3 - Scrittura di {} messaggi Streams nel canale privato - Devnet\n", NUM_MESSAGGI);
    test_3::scrittura_messaggi_stream::test_3(coordinate.clone(), url_devnet, String::from("./risultati/risultato_t3_devnet.csv"), String::from("./risultati/risultato_t4_devnet.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nTest 3 - Scrittura di {} messaggi Streams nel canale privato - Mainnet\n", NUM_MESSAGGI);
    test_3::scrittura_messaggi_stream::test_3(coordinate.clone(), url_mainnet, String::from("./risultati/risultato_t3_mainnet.csv"), String::from("./risultati/risultato_t4_mainnet.csv")).await?;

    println!();
    Ok(())
}
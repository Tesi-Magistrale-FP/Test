use anyhow::Result;

mod test_1;
mod test_2;
mod utility;

const NUM_MESSAGGI: i32 = 100;

#[tokio::main]
async fn main() -> Result<()>
{
    let url_devnet: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet = String::from("https://chrysalis-nodes.iota.org");

    println!("\nTest 1-2 - Generazione casuale delle coordinate\n");
    let coordinate = utility::generazione_casuale::genera_coordinate(NUM_MESSAGGI);
    println!("- {} coordinate generate con successo", NUM_MESSAGGI);

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} messaggi dati sul Tangle - Devnet\n", NUM_MESSAGGI);
    test_1::scrittura_messaggi_dati::test_1(coordinate.clone(), url_devnet, String::from("./risultati/risultato_t1_devnet.csv"), String::from("./risultati/risultato_t2_devnet.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} messaggi dati sul Tangle - Mainnet\n", NUM_MESSAGGI);
    test_1::scrittura_messaggi_dati::test_1(coordinate.clone(), url_mainnet, String::from("./risultati/risultato_t1_mainnet.csv"), String::from("./risultati/risultato_t2_mainnet.csv")).await?;

    println!();
    Ok(())
}
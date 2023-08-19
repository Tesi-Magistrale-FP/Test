use anyhow::Result;

mod test_1_2;
mod utility;

const NUM_TRANSAZIONI: i32 = 100;

#[tokio::main]
async fn main()  -> Result<()>
{
    let url_devnet: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet = String::from("https://chrysalis-nodes.iota.org");

    println!("\nTest 1 - Generazione casuale delle coordinate\n");
    let coordinate = utility::generazione_casuale::genera_coordinate(NUM_TRANSAZIONI);
    println!("- {} coordinate generate con successo", NUM_TRANSAZIONI);

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} messaggi dati sul Tangle - Devnet\n", NUM_TRANSAZIONI);
    test_1_2::scrittura_lettura_messaggi_dati::test_1_2(coordinate.clone(), url_devnet, String::from("./risultati/risultato_t1_devnet.csv"), String::from("./risultati/risultato_t2_devnet.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} messaggi dati sul Tangle - Mainnet\n", NUM_TRANSAZIONI);
    test_1_2::scrittura_lettura_messaggi_dati::test_1_2(coordinate.clone(), url_mainnet, String::from("./risultati/risultato_t1_mainnet.csv"), String::from("./risultati/risultato_t2_mainnet.csv")).await?;

    println!();
    Ok(())
}
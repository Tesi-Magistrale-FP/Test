use anyhow::Result;

mod test_1;
mod utility;

const NUM_TRANSAZIONI: i32 = 1;

#[tokio::main]
async fn main()  -> Result<()>
{
    let url_devnet: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet = String::from("https://chrysalis-nodes.iota.org");

    println!("\nTest 1 - Generazione delle coordinate\n");
    let coordinate = utility::generazione_casuale::genera_coordinate(NUM_TRANSAZIONI);
    println!("- {} coordinate generate con successo", NUM_TRANSAZIONI);

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} transazioni semplici sul Tangle - Devnet\n", NUM_TRANSAZIONI);
    test_1::post_messages::test_1(coordinate.clone(), url_devnet, String::from("./risultati/risultato_t1_devnet.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nTest 1 - Scrittura di {} transazioni semplici sul Tangle - Mainnet\n", NUM_TRANSAZIONI);
    test_1::post_messages::test_1(coordinate.clone(), url_mainnet, String::from("./risultati/risultato_t1_mainnet.csv")).await?;

    println!();
    Ok(())
}
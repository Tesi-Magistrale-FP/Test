use anyhow::Result;

mod test_1;
mod utility;

const NUM_TRANSAZIONI: i32 = 2;

#[tokio::main]
async fn main()  -> Result<()>
{
    let url_devnet: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet = String::from("https://chrysalis-nodes.iota.org");

    println!("\nGenerazione delle coordinate\n");
    let coordinate = utility::generazione_casuale::genera_coordinate(NUM_TRANSAZIONI);
    println!("- {} coordinate generate con successo", NUM_TRANSAZIONI);

    println!("\n---------------------------------------");
    println!("\nScrittura di {} transazioni semplici sul tangle - Devnet\n", NUM_TRANSAZIONI);
    test_1::test_1::test(coordinate.clone(), url_devnet, String::from("./risultati/risultato_t1_devnet.csv")).await?;

    println!("\n---------------------------------------");
    println!("\nScrittura di {} transazioni semplici sul tangle - Mainnet\n", NUM_TRANSAZIONI);
    test_1::test_1::test(coordinate.clone(), url_mainnet, String::from("./risultati/risultato_t1_mainnet.csv")).await?;

    Ok(())
}
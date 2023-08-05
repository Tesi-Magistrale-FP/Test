use anyhow::Result;

mod test_info;
mod test_1;

#[tokio::main]
async fn main()  -> Result<()>
{
    println!("\nInformazioni sul nodo - Devnet\n");
    test_info::test::node_info_dev().await?;

    println!("\n---------------------------------------");
    println!("\nInformazioni sul nodo - Mainnet\n");
    test_info::test::node_info_main().await?;

    println!("\n---------------------------------------");
    println!("\nTest 1 - Attaccare varie transazioni semplici al tangle - Devnet\n");
    test_1::test::test_tx_dev().await?;

    println!("\n---------------------------------------");
    println!("\nTest 1 - Attaccare varie transazioni semplici al tangle - Mainnet\n");
    test_1::test::test_tx_main().await?;

    Ok(())
}
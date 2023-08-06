use anyhow::Result;

mod node_info;

#[tokio::main]
async fn main()  -> Result<()>
{
    let url_devnet_1: String = String::from("https://api.lb-0.h.chrysalis-devnet.iota.cafe/");
    let url_devnet_2: String = String::from("https://api.lb-1.h.chrysalis-devnet.iota.cafe/");
    let url_mainnet_1 = String::from("https://chrysalis-nodes.iota.org");
    let url_mainnet_2: String = String::from("https://chrysalis-nodes.iota.cafe");

    println!("\nInformazioni sul nodo - Devnet #1\n");
    node_info::node_info::get_node_info(url_devnet_1.clone()).await?;

    println!("\n---------------------------------------");
    println!("\nInformazioni sul nodo - Devnet #2\n");
    node_info::node_info::get_node_info(url_devnet_2.clone()).await?;

    println!("\n---------------------------------------");
    println!("\nInformazioni sul nodo - Mainnet #1\n");
    node_info::node_info::get_node_info(url_mainnet_1.clone()).await?;

    println!("\n---------------------------------------");
    println!("\nInformazioni sul nodo - Mainnet #2\n");
    node_info::node_info::get_node_info(url_mainnet_2.clone()).await?;

    println!();
    Ok(())
}
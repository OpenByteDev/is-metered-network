fn main() -> std::io::Result<()> {
    println!("{}", is_metered_network::connection_cost()?);
    Ok(())
}

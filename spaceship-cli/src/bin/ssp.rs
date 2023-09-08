use anyhow::Context;

fn main() -> anyhow::Result<()> {
    println!("spaceship");
    do_thing()?;

    Ok(())
}

fn do_thing() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("../lib.rs").context("Could not read the lib.rs file")?;

    Ok(())
}
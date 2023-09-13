use anyhow::Context;

fn main() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().unwrap();
    println!("Spaceship CLI v0.0.1 - dir {}", cwd.display());
    do_thing()?;

    Ok(())
}

fn do_thing() -> anyhow::Result<()> {
    let data = std::fs::read_to_string("./spaceship-cli/src/lib.rs")
        .context("Could not read the lib.rs file")?;


    Ok(())
}
use xshell::cmd;

fn main() -> xshell::Result<()> {
    let sh = xshell::Shell::new()?;
    eprintln!("Building rust:");
    cmd!(sh, "cargo build --quiet --release --bin ping-pong").run()?;
    eprintln!("\nBuilding Go:");
    cmd!(sh, "go build main.go").run()?;

    eprintln!("\nRunning Rust:");
    cmd!(sh, "./target/release/ping-pong").run()?;
    cmd!(sh, "taskset --cpu-list 1 ./target/release/ping-pong").run()?;

    eprintln!("\nRunning Go:");
    cmd!(sh, "./main").run()?;
    cmd!(sh, "taskset --cpu-list 1 ./main").run()?;
    Ok(())
}

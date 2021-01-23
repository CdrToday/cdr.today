use structopt::StructOpt;

fn main() -> poc::Result<()> {
    poc::Opt::from_args().exec()?;
    Ok(())
}

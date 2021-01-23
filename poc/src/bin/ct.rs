use structopt::StructOpt;

fn main() -> poc::result::Result<()> {
    poc::Opt::from_args().exec()?;
    Ok(())
}

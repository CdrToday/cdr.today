use structopt::StructOpt;

#[actix_web::main]
async fn main() -> poc::Result<()> {
    poc::Opt::from_args().exec().await?;
    Ok(())
}

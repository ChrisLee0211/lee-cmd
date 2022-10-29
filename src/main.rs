use clap::{Parser, Subcommand, Args};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Commands {
    #[command(subcommand)]
    action: Action
}

#[derive(Subcommand, Debug)]
enum Action {
    /// git pull origin xxx
    PL(Branch),
    /// git push origin xxx
    PH(Branch)
}
#[derive(Args, Debug)]
struct Branch {
    #[arg(short, required = false)]
    branch: Option<String>,
}


fn main() {
    let cli = Commands::parse();

    match &cli.action {
        Action::PH(Branch { branch }) => {
            println!("git push {:?}", &branch)
        }
        Action::PL(Branch { branch }) => {

        }
    }
}
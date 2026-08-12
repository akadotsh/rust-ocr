use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   pattern: String
}
fn main() {
      let args = Cli::parse();
      print!("args: {:?}",args,)
}

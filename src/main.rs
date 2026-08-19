use clap::Parser;
use leptess::LepTess;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   path: String
}
fn main() {
      let cli = Cli::parse();
      let mut lt =  LepTess::new(None,"eng").unwrap();
      lt.set_image(&cli.path)
        .unwrap();
      println!("{}", lt.get_utf8_text().unwrap());

}

mod options;

use options::Options;
use passwords::PasswordGenerator;
use structopt::StructOpt;

fn main() {
    let options = Options::from_args();
    let pg: PasswordGenerator = options.into();
    println!("{}", pg.generate_one().unwrap());
}

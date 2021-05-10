use structopt::StructOpt;

mod model;
mod opt;
mod string_supplements;

fn main() {
    let opt = opt::Opt::from_args();
    println!("{:?}", opt);
}

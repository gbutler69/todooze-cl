use structopt::StructOpt;

mod opt;

fn main() {
    let opt = opt::Opt::from_args();
    println!("{:?}", opt);
}

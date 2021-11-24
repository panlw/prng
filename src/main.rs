use clap::Parser;
use rand::{thread_rng, Rng};

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Parser)]
#[clap(version = "0.1.0", author = "Neo Pan <neopan@126.com>")]
struct Opts {
  #[clap(short, long, parse(from_occurrences))]
  verbose: i32,
  #[clap(short, long, default_value = "5")]
  length: usize,
}

fn main() {
  let opts: Opts = Opts::parse();
  // println!("{:?}", opts);

  let mut buf: Vec<u8> = Vec::new();
  buf.resize(opts.length, 0);

  let mut rng = thread_rng();
  rng.fill(buf.as_mut_slice());
  // print_type_of(&buf);

  let hex = hex::encode(buf);
  println!("{}", hex);
}

mod bit;
mod modulo;
mod number_theoretic_transform;
mod p_values;
mod p_values_modulo;

use bytes::BufMut;
use clap::{Arg, Command};
use std::fs::File;
use std::io::Write;

fn main() {
    let matches = Command::new("MyApp")
        .about("calculates the p-values")
        .arg(
            Arg::new("sample size")
                .short('s')
                .long("size")
                .takes_value(true)
                .required(true)
                .value_name("N")
                .help("Specify the sample size"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("out")
                .takes_value(true)
                .required(true)
                .value_name("FILE")
                .help("Specify the name of file to output"),
        )
        .get_matches();

    let sample_size = match matches.value_of("sample size").unwrap().parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("sample size must be a number ({})", err);
            return;
        }
    };
    let filename = matches.value_of("output").unwrap();
    let mut file = match File::create(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("failed to open `{}` ({})", filename, err);
            return;
        }
    };

    let mut buf = Vec::new();
    for row in p_values::p_values(sample_size) {
        for p in row {
            buf.put_f64(p);
        }
    }

    // 1 個 8 バイト，
    // 0 から sample_size までで sample_size + 1 個
    // rho1 と rho2 で 2 つ
    assert_eq!(buf.len(), 8 * (sample_size as usize + 1) * 2);

    file.write_all(&buf).expect("failed to write");
    file.flush().expect("failed to flush");
}

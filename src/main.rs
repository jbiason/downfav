use elefren::prelude::*;
use elefren::helpers::cli;

fn main() {
    let registration = Registration::new("https://functional.cafe")
        .client_name("downfav")
        .build()
        .unwrap();
    let client = cli::authenticate(registration).unwrap();

    println!("{:?}", client
             .get_home_timeline().unwrap()
             .items_iter()
             .take(100)
             .collect::<Vec<_>>()
             );
}

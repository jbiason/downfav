use elefren::prelude::*;
use elefren::helpers::cli;

fn main() {
    let registration = Registration::new("https://functional.cafe")
        .client_name("downfav")
        .build()
        .unwrap();
    let client = cli::authenticate(registration).unwrap();

    client
        .favourites().unwrap()
        .items_iter()
        .take(2)
        .for_each(move |record| println!("{:#?}", record))
        ;
}

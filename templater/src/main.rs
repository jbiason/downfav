use clap::App;
use clap::Arg;
use clap::crate_name;
use clap::crate_version;
use clap::crate_authors;
use clap::crate_description;

fn main() {
    let params = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("bind")
            .short("b")
            .long("bind")
            .value_name("ADDRESS")
            .help("Binding address for the service")
            .takes_value(true))
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Port to bind")
            .takes_value(true))
        .get_matches();
    println!("Hello, world!");
}

use clap::{App, Arg};
use pcr_eventlog_attestation::{server::server, VERSION};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("pea-server")
        .version(VERSION)
        .about("Server for authenticating itself off PCRs measurement")
        .arg(
            Arg::with_name("eventlog")
                .short("e")
                .long("eventlog")
                .value_name("FILE")
                .help("The file storing the eventlog")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("listen")
                .short("l")
                .long("listen")
                .value_name("LISTEN")
                .help("The addr:port to run server onto")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("image-id")
                .short("i")
                .long("image-id")
                .value_name("IMAGE-ID")
                .help("The image we're running on")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let listen = matches.value_of("listen").unwrap();
    let eventlog = matches.value_of("eventlog").unwrap();
    let imageid = matches.value_of("image-id").unwrap();

    server(listen, eventlog, imageid).await.map_err(|e| {
        eprintln!("error: {}", e);
        e
    })?;
    Ok(())
}

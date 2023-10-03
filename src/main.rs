use escpos_rs::{Printer, PrinterProfile};
use axum::{routing::post, Router, extract::Extension, http::StatusCode};
use std::net::SocketAddr;
use clap::{Parser};
use clap_num::{maybe_hex};
use std::sync::Arc;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///HTTP port to listen to.
    #[clap(short, long, default_value = "8080")]
    port: u16,

    ///Device vendor ID in lsusb command
    #[clap(short, long, required=true, value_parser=maybe_hex::<u16>)]
    vendor_id: u16,

    ///Device product ID in lsusb command
    #[clap(short, long, required=true, value_parser=maybe_hex::<u16>)]
    device_id: u16,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let printer = Arc::new(Printer::new( PrinterProfile::usb_builder(
        cli.vendor_id,
        cli.device_id
    ).build()).unwrap().unwrap());

    let app = Router::new()
        .route("/", post(handler))
        .layer(Extension(printer));

    let addr = SocketAddr::from(([127, 0, 0, 1], cli.port));

    println!("Raw Printer server running at: http://localhost:{}", cli.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(
    printer: Extension<Arc<Printer>>,
    body: String,
) -> (StatusCode, String) {
    match printer.print(&body) {
        Ok(()) => (StatusCode::OK, String::new()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

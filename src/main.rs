mod extra;

use std::error::Error;
use escpos_rs::{Printer, PrinterProfile};
use clap::{Parser};
use clap_num::{maybe_hex};
use tiny_http::{Server, Response};
use extra::escape;

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    ///HTTP port to listen to.
    #[clap(short, long, default_value = "8001")]
    port: u16,

    ///Device vendor ID in lsusb command
    #[clap(short, long, required=true, value_parser=maybe_hex::<u16>)]
    vendor_id: u16,

    ///Device product ID in lsusb command
    #[clap(short, long, required=true, value_parser=maybe_hex::<u16>)]
    device_id: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let printer = Printer::new(PrinterProfile::usb_builder(
        cli.vendor_id,
        cli.device_id
    ).build())?.ok_or("no device found!")?;

    let host = format!("localhost:{}", cli.port);

    let server = match Server::http(&host) {
        Ok(server) => server,
        Err(msg) => {
            return Err(msg.to_string().into());
        }
    };

    println!("Raw Printer server running at: http://{}", host);
    for mut request in server.incoming_requests() {
        let method = request.method().to_string();
        let url = request.url().to_string();
        let mut body = String::new();

        request.as_reader().read_to_string(&mut body)?;
        request.respond(
            if &method == "POST" && &url == "/" {
                match printer.print(&escape(body)) {
                    Ok(()) => Response::from_string("").with_status_code(200),
                    Err(err) => Response::from_string(&err.to_string())
                        .with_status_code(500)
                }
            } else {
                Response::from_string("").with_status_code(404)
            }
        )?;
    }
    Ok(())
}

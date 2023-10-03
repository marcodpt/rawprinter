# Raw Printer
An IOT server written in rust for connecting via USB to raw printers.

## Usage Example
As an example, I will show how to connect the `Zebra` `TLP 2844` printer that
I used to test the server.

First you need to identify the `vendor-id` and `device-id` of the printer.

Run the command in the Linux terminal, or similar command in another operating
system.
```
lsusb | grep Zebra
```

On my machine I get the following result:
```
Bus 002 Device 008: ID 0a5f:000a Zebra TLP2844
```

So to start the server you must run the command:
```
sudo rawprinter --vendor-id 0x0a5f --device-id 0x000a
```

If you are compiling manually:
```
sudo ./target/release/rawprinter --vendor-id 0x0a5f --device-id 0x000a
```

## Testing
To send a test print I am using [hurl](https://hurl.dev/):
```
hurl --test test.hurl
```

## Contributing
It's a very simple project.
Any contribution, any feedback is greatly appreciated.

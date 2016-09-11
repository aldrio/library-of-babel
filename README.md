# The Library of Babel

A Rust implementation of [The Library of Babel](https://libraryofbabel.info/).

## Usage

Pages are identified in the library by the wall, shelf, volume, page number in base10, and room address in base64 seperated by colons.

`wall:shelf:volume:page:hex_address`

Explore the Library using the `search` and `read` commands.

#### Search
Search for strings keeping in mind that the query must be all lowercase a-z, comma, period, or space.
```sh
babel search "hello world"
# Or if running straight from cargo:
cargo run -- search "hello world"
```




#### Read
Pass in a library location with the format `wall:shelf:volume:page:hex_address` to the `read` subcommand.
```sh
babel read 2:4:18:310:W3IRDhEX_n4me
# Cargo:
cargo run -- read 2:4:18:310:W3IRDhEX_n4me
```
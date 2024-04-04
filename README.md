# DO NOT USE

You probably shouldn't use this tool, I only tested it for myself and I'm just learning Rust.

# translate-ethstaker-to-bitcoin-tax
This is a simple script to trasnlate between the csv format (using `;` as delimiter) output by ethstaker.tax and convert it into a csv format that can be input into bitcoin.tax.

I mostly made this to practice rust and procrastinate my taxes...

## Usage

This script reads csv input on stdin and writes to stdout. It expects either a rocketpool-mode output or a standard mode output.

`cat eth_staker_tax.csv | cargo run translate-ethstaker-to-bitcoin-tax > bitcoint_tax.csv`

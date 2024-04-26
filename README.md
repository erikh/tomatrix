# Matrix any text document

Turns it into a full screen display that'll make you say "whoa."

## Usage

Pipe content to stdin and close it, and the saver will start.

e.g., `cargo run < src/lib.rs`. Play with the character sets to influence the output, e.g., `/usr/share/dict/words` vs some code.

`cargo install tomatrix` to install this.

This one is fun: `head -c 1000000 /dev/urandom | perl -pe 's/\P{Word}//g' | tomatrix`

Press Control+C to exit.

## Example

<img src="example.png" />

## Author

The Professional <erik@hollensbe.org>

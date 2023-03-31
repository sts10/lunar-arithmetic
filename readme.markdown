# Lunar Arithmetic 

Earlier today the all-knowing YouTube algorithm took me to [this Numberphile video about Lunar Arithmetic](https://www.youtube.com/watch?v=cZkGeR9CWbk). Here's [the entry in the Online Encyclopedia of Integer Sequences](https://oeis.org/A087061), which comments: 

> There are no carries in lunar arithmetic. For each pair of lunar digits, to Add, take the lArger, but to Multiply, take the sMaller. For example: 169 + 248 = 269 and 169 * 248 = 12468

The entry also notes:

> We have changed the name from "dismal arithmetic" to "lunar arithmetic" - the old name was too depressing. - N. J. A. Sloane, Aug 06 2014

I thought it'd be fun to write the relevant mathematical functions in Rust. Run `cargo test` to test the functions.


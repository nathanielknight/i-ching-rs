# I Ching Rust

This is a [Rust] program that provides a web interface for [casting the I
Ching]. The URL of each result has all the information that went into producing
it, so you can copy-paste or bookmark it if you want to be able to come back to
a particular cast.

The program is provided under the terms [Prosperity Public License 3.0].
See [LICENSE.md] for the full text of the license.

[Rust]: https://www.rust-lang.org/
[casting the I Ching]: https://en.wikipedia.org/wiki/I_Ching_divination
[Prosperity Public License 3.0]: https://prosperitylicense.com/versions/3.0.0
[LICENSE.md]: ./blob/main/LICENSE.md

# Method

To generate a hexagram, this program

* collects a prompt and the current date,
* hashes them with SHA256,
* seeds a random number generator with that hash, and
* simulates the [three-coin method] for casting the I Ching.

[three-coin method]: https://en.wikipedia.org/wiki/I_Ching_divination#Three-coin_method

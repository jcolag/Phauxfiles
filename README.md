Phauxfiles
==========

Goofy tool to generate web pages that might look like user profiles returned by a search.

In reality, _Phauxfiles_ isn't much more than an excuse to experiment with [Rust](http://rust-lang.org/), but thanks to the good graces of [User Inter Faces](http://uifaces.com/) and [UI Names](http://uinames.com/) and their public APIs, it's a mildly entertaining hack.

By far, the biggest roadblock for this project has been that there doesn't appear to be a solid HTTP library for Rust.  One (rust-http) refers to itself as obsolete, its successor (Teepee) calls itself "completely unusable" and seems abandoned, and the third (Hyper) didn't build for me.  Fortunately, the basics of sending an HTTP request and pulling the body out of an HTTP response aren't exactly rocket science.  So, that's a stopgap.  When one pulls ahead, I'll try to replace my HTTP code and wipe this paragraph.

The program itself is fairly straightforward:  Grab a bunch of names from UI Names and, for each name, get a URL to a random avatar from User Inter Faces.  Wrap that in style-ready HTML and print it.

Build
-----

Ready?

    cd phauxfiles
    cargo build

And there you go.

Run
---

Sure, from the `phauxfiles` folder, you _could_ try...

    cargo run

Alternatively, you can try...

    ./target/phauxfiles

...which is the same thing, _except_ that you can add a command-line parameter to specify how many fake profiles will be generated and whether to output to a specific file or served across HTTP, such as...

|**Short Form**|**Long Form**|**Type** |**Default**|**Description**|
|:------------:| ----------- |:-------:|:---------:| ------------- |
| -h | --help                |         |           | Print help information |
| -n | --number-of-profiles  | integer | 6         | Number of profiles to include |
| -o | --output-file         | string  |           | Name of output file |
| -s | --server              | integer |           | Serves the page from an HTTP server on the port |

If an output file is specified, _Phauxfiles_ overwrites it with the output.  If none is specified, it prints the output to `stdout`.

There is a sample CSS file in the folder referenced by the HTML output, so the resulting output can be opened in a web browser directly.

Please note that there appears to be a conflict between `getopts` and execution via `cargo run`.

Caveats
-------

In the spirit of full disclosure...

 - As mentioned above, this _really_ should be written with a solid HTTP library.  It's a safe bet that I'm not implementing nearly enough of the specification for robust reactions to error conditions, let alone being able to negotiate with the servers if it ever becomes necessary.

 - At some point, I upgraded the Rust compiler.  It now has trouble with my use of `deriving(Encodable)`, but doesn't appear to yet know how to use the replacement, `deriving(RustcEncodable)`.  And `decodable`, of course.

 - It might also be nice to be able to switch between HTML and text mode.

 - More extensive profile information would be nice, though there don't seem to be any handy APIs to play with.  I may integrate quotes and cat pictures at some point, but that still doesn't flesh out the profile itself.


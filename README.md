Phauxfiles
==========

Goofy tool to generate web pages that might look like user profiles returned by a search.

In reality, _Phauxfiles_ isn't much more than an excuse to experiment with [Rust](http://rust-lang.org/), but thanks to the good graces of [User Inter Faces](http://uifaces.com/) and [UI Names](http://uinames.com/) and their public APIs, it's a mildly entertaining hack.

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

There is a sample CSS file in the folder referenced by the HTML output, so the resulting output can be opened in a web browser directly.  In "server mode," the CSS file will be served out of the current directory.  Files are "white-listed," so there is no security concern unless you edit `format.css` to include information that should be secure.

Please note that there appears to be a conflict between `getopts` and execution via `cargo run`.

Web Server
----------

Running the web server allows a more flexible usage, by parsing the URL query parameters.

|**Name**|**Type** |**Description**|
|:------:|:-------:| ------------- |
| count  | integer | Number of profiles to include   |
| where  | string  | Country of origin for the names |
| sex    | string  | Gender to select for the names  |

The `sex` parameter is validated by Phauxfiles as one of `m`, `M`, `male`, `f`, `F`, or `female`.

The `where` parameter is also validated, turning ISO country codes into country names known to be valid for UINames, which include the following:

|**Abbreviation**|**Country** |
|:--------------:|:---------  |
| al | Albania                |
| ar | Argentina              |
| au | Australia              |
| at | Austria                |
| az | Azerbaijan             |
| bd | Bangladesh             |
| be | Belgium                |
| ba | Bosnia and Herzegovina |
| br | Brazil                 |
| ca | Canada                 |
| cn | China                  |
| co | Colombia               |
| dk | Denmark                |
| eg | Egypt                  |
| gb | England                |
| fi | Finland                |
| fr | France                 |
| ge | Georgia                |
| de | Germany                |
| gr | Greece                 |
| hu | Hungary                |
| in | India                  |
| ir | Iran                   |
| il | Israel                 |
| it | Italy                  |
| jp | Japan                  |
| mx | Mexico                 |
| ma | Morocco                |
| nl | Netherlands            |
| nz | New Zealand            |
| ng | Nigeria                |
| pl | Poland                 |
| pt | Portugal               |
| ro | Romania                |
| ru | Russia                 |
| es | Spain                  |
| se | Sweden                 |
| ch | Switzerland            |
| tr | Turkey                 |
| ua | Ukraine                |
| us | United States          |
| vn | Vietnam                |

Note that using the country's full name is no longer accepted by Phauxfiles and the abbreviations must currently be lowercase.  If the code is not recognized, it is eliminated, reverting to the default random behavior.

Assuming that the server listens on port 8080, an example usage might be <`http://localhost:8080?count=5&where=in&sex=f`>, giving five random female Indian names with random avatar images.

Caveats
-------

In the spirit of full disclosure...

 - The program (rightly) now relies on the hyper library.  It's currently set to use the latest version, which may require some minor updates as the library improves.

 - It might also be nice to be able to switch between HTML and text mode.

 - More extensive profile information would be nice, though there don't seem to be any handy APIs to play with.  I may integrate quotes and cat pictures at some point, but that still doesn't flesh out the profile itself.


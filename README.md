This is a patched version of the latest CRATES.IO package to work on windows.
(this is NOT a patch of the gitlab version which works differently regarding dependencies,
with dependencies coming from crates.io for this version and git for the gitlab version,
pulling things from branches of the git repo `gtk-rs-core`)

# poppler-rs

A high-level (safe) set of Rust bindings for
[poppler](https://poppler.freedesktop.org/)'s glib interface.

poppler is a PDF rendering library with a cairo backend.

## Usage

There's no nice tutorial or sample code for now: check out the rustdoc to see
what types are available.

But generally, usage involves:

  * Creating a `Document`, maybe from a `&[u8]` slice.
	* Enumerating pages, rendering them to a cairo surface

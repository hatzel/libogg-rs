# Safe Rust Bindings for libogg

Simple bindings for libogg.

In most cases it should be preferable to use the [ogg crate](https://github.com/RustAudio/ogg).
However, their README states that the encoder is not well tested.
It does at the time of writing not offer some options (e.g. setting a stream id).
Still, you are probably better off using `ogg` unless it's a very special case.

These bindings are not zero-copy!
The functions producing a page don't return a reference as modeling their
invalidation would make the API more complicated.

## Simple Example

```rust
extern crate libogg;
use libogg::{Packet, Stream};

fn main() {
    let mut stream = Stream::new(1234); // provide a unique stream id

    // This loop puts data into the stream until a page is returned
    loop {
        match stream.pageout() {
            Some(page) => {
                // `page` holds a header and body, write them to a file
                println!("header: {:?}/body {:?}", page.header, page.body);
                break;
            },
            None => {
                let mut vec = Vec::new();
                for x in 0..255 {
                    vec.push(x);
                }
                // Push in new data
                let mut pkt = Packet::new(&mut vec);
                stream.packetin(&mut pkt)
            }
        }
    }
}
```

## Documentation

This crate has some sporadic documentation, check out the [libogg docs](https://xiph.org/ogg/doc/libogg/)
for details.

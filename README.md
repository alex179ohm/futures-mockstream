<h1 align="center">Futures MockStream</h1>
<div align="center">
  <strong>
    A simple MockStream for <a href="https://docs.rs/futures/latest/futures/io/trait.AsyncRead.html">AsyncRead</a>, <a href="https://docs.rs/futures/latest/futures/io/trait.AsyncWrite.html">AsyncWrite</a> and <a href="https://docs.rs/futures/latest/futures/stream/trait.Stream.html">Stream</a> traits from the <a href="https://crates.io/crates/futures">futures</a> crate.

  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/alex179ohm/futures-mockstream/actions?query=workflow%3ABuild">
    <img src="https://github.com/alex179ohm/futures-mockstream/workflows/Build/badge.svg"
    alt="Tests status" />
  </a>
  <a href="https://github.com/alex179ohm/futures-mockstream">
    <img alt="Crates.io" src="https://img.shields.io/crates/l/futures-mockstream">
  </a>
  <a href="https://crates.io/crates/futures-mockstream">
    <img src="https://img.shields.io/crates/v/futures-mockstream.svg?style=flat-square"/>
  </a>
  <a href="https://docs.rs/futures-mockstream">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href='https://coveralls.io/github/alex179ohm/futures-mockstream?branch=master'>
    <img src='https://coveralls.io/repos/github/alex179ohm/futures-mockstream/badge.svg?branch=master' alt='Coverage Status' />
  </a>
</div>

Futures\_mockstream allows Stream implementations to be tested without a real tcp/tls server or client.

# Install


Install from [crates.io](https://crates.io)


```
[dependencies]
futures-mockstream = "0.1.2"
```


# Example


with [smol](https://github.com/stjepang/smol)


```rust
use futures_mockstream::MockStream;
use smol;


#[cfg(test)]
mod tests {
    #[test]
    fn async_read() {
        smol::run(async {
            let mut mockstream = MockStream::from(&b"GET /index HTTP/1.1\r\n");
            while let Some(resp) = MyConn::with_stream(mockstream).next().await {
                match resp {
                    Ok(r) => { // your asserts },
                    Err(e) => {},
                }
            }
        })
    }
}
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


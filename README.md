<h1 align="center">Futures MockStream</h1>
<div align="center">
  <strong>
    MockStream which implements <a href="https://docs.rs/futures/latest/futures/io/trait.AsyncRead.html">AsyncRead</a>, <a href="https://docs.rs/futures/latest/futures/io/trait.AsyncWrite.html">AsyncWrite</a> and <a href="https://docs.rs/futures/latest/futures/stream/trait.Stream.html">Stream</a> trais from the <a href="https://crates.io/crates/futures">futures</a> crate.

  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/alex179ohm/futures-mockstream/actions?query=workflow%3ABuild">
    <img src="https://github.com/alex179ohm/futures-mockstream/workflows/Build/badge.svg"
    alt="Tests status" />
  </a>

  <a href="https://crates.io/crates/futures-mockstream">
    <img src="https://img.shields.io/crates/v/futures-mockstream.svg?style=flat-square"/>
  </a>
  <a href="https://docs.rs/futures-mockstream">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

</div>



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

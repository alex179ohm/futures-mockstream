<h1 align="center">Futures MockStream</h1>
<div align="center">
  <strong>
    MockStream for <a href="https://crates.io/crates/futures"futures</a> crate Async{Read, Write} and Stream traits

  </strong>
</div>

<br />

<div align="center">
  <a href="https://github.com/alex179ohm/futures-mockstream/actions?query=workflow%3ABuild">
    <img src="https://github.com/wolf4ood/wait-for-me/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>

  <a href="https://crates.io/crates/futures-mockstream">
    <img src="https://img.shields.io/crates/d/futures-mockstream.svg?style=flat-square"
      alt="Download" />
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
futures-mockstream = "0.1"
```


# Example


with [smol](https://github.com/stjepang/smol)


```rust
use futures-mockStream::MockStream;
use smol;


#[cfg(test)]
mod tests {
    #[test]
    fn async_read() {
        let mut mockstream = MockStream::from(&b"GET /index HTTP/1.1\r\n");
        while let Some(resp) = MyConn::with_stream(mockstream).next().await {
            match resp {
                Ok(r) => { // your asserts },
                Err(e) => {},
            }
        }
    }
}
```

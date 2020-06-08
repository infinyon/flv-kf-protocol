<h1 align="center">flv-kf-protocol</h1>
<div align="center">
 <strong>
   Native Rust implementation of kafka protocol.
 </strong>
</div>

<div align="center">
   <!-- CI status -->
  <a href="https://github.com/infinyon/flv-kf-protocol/actions">
    <img src="https://github.com/infinyon/flv-kf-protocol/workflows/CI/badge.svg"
      alt="CI Status" />
  </a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/kf-protocol">
    <img src="https://img.shields.io/crates/v/kf-protocol?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/kf-protocol">
    <img src="https://img.shields.io/crates/d/kf-protocol.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/kf-protocol">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
</div>



# Testing Client

To test command to test spu server:

```
> ./send-b-client.sh data/apirequest.txt 9004
```

# dump kafka binary as pretty print

First build dump binary:

```cargo build --features="cli"```

Then run binary:
```../target/debug/kafka-dump```



## License

This project is licensed under the [Apache license](LICENSE-APACHE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Fluvio by you, shall be licensed as Apache, without any additional
terms or conditions.

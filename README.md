# tracing-attributes-http

Macro attributes for HTTP tracing.

## Examples

All examples use the Jaeger UI for viewing distributed span and trace data.
To setup a Jaeger instance locally

```bash
podman run -p6831:6831/udp -p6832:6832/udp -p16686:16686 jaegertracing/all-in-one:latest
```

The traced data can be browsed at [`http://localhost:16686`](http://localhost:16686).

### Echo

The upstream `echo` example setup to be traceable, using the [`tracing`]() crate.

```bash
cargo run --features="full traceable tracing/max_level_trace" --example echo_traceable
```

In a second console/shell:

```bash
curl localhost:3000/echo -XPOST -d 'hello world'
curl localhost:3000/echo -XPOST -d 'hello world 10'
curl localhost:3000/echo -XPOST -d 'hello world 100'
firefox http://localhost:16686
```

## Developers

To run tests:

```bash

```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0)) or

- MIT license ([LICENSE-MIT](http://opensource.org/licenses/MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

### Dracarys

Dracarys is a [Kong](https://getkong.org) API gateway client in Rust. The aim is to provide an elegant API to interact
with Kong for managing Kong APIs, plugins and Kong clusters.

**Dracarys** is a work in progress as such isn't ready for [crates.io](http://crates.io) yet but I'm working
round the clock.

#### Use `git` dependency in your `Cargo.toml`:

```ini
[dependencies]
http = { git = 'https://github.com/younisshah/dracarys' }
```


#### Example usage to add an API to Kong

```rust
extern crate dracarys;

use dracarys;

fn main() {
    let kong = dracarys::roar::Kong::new("http://127.0.0.1:8001");
    let api = dracarys::types::KongAPI {
        name: "roar".to_string(),
        hosts: vec!["httpbin.org".to_string()],
        uris: vec![],
        preserve_host: false,
        upstream_url: "http://httpbin.org".to_string(),
        https_only: false,
        http_if_terminated: true,
    };
    println!("Result {}", kong.add_api(&api).unwrap());
}
```

#### Why "dracarys"?

Because GOT!

#### TODO

- [ ] Write sane documentation.
- [ ] Make a crate.
- [ ] Retrieve node information and status.
- [ ] Handle Kong clusters.
- [ ] Consume Admin Add API.
- [ ] Add API consumers.
- [ ] Add/update/list plugins.

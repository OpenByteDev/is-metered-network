# is-metered-network

[![CI](https://github.com/OpenByteDev/is-metered-network/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/is-metered-network/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/is-metered-network.svg)](https://crates.io/crates/is-metered-network)
[![Documentation](https://docs.rs/is-metered-network/badge.svg)](https://docs.rs/is-metered-network)
[![dependency status](https://deps.rs/repo/github/openbytedev/is-metered-network/status.svg)](https://deps.rs/repo/github/openbytedev/is-metered-network)
[![MIT](https://img.shields.io/crates/l/is-metered-network.svg)](https://github.com/OpenByteDev/is-metered-network/blob/master/LICENSE)

Cross-platform detection of whether the current network connection is metered, i.e. whether data usage may be limited or billed (cellular data, a mobile hotspot, a capped plan, ...).

| Platform | Source                                                                                                                                                             |
| -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Windows  | the Network List Manager COM API ([`INetworkCostManager::GetCost`](https://learn.microsoft.com/en-us/windows/win32/api/netlistmgr/nf-netlistmgr-inetworkcostmanager-getcost)) |
| Linux    | the [`Metered`](https://networkmanager.dev/docs/api/latest/gdbus-org.freedesktop.NetworkManager.html) property of `org.freedesktop.NetworkManager` on the D-Bus system bus |

macOS is not supported yet.

## Usage
```rust
if is_metered_network::check()? {
    println!("on a metered connection");
}
```

For the underlying cost instead of a plain bool (e.g. to distinguish a known-unmetered connection from an unknown one), use [`connection_cost`](https://docs.rs/is-metered-network/latest/is_metered_network/fn.connection_cost.html):
```rust
use is_metered_network::ConnectionCost;

match is_metered_network::connection_cost()? {
    ConnectionCost::Unmetered => println!("unmetered"),
    ConnectionCost::Metered => println!("metered"),
    ConnectionCost::Unknown => println!("unknown"),
}
```

## License
Licensed under MIT license ([LICENSE](https://github.com/OpenByteDev/is-metered-network/blob/master/LICENSE) or http://opensource.org/licenses/MIT)

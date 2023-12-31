<!-- SPDX-FileCopyrightText: The cirque authors -->
<!-- SPDX-License-Identifier: MPL-2.0 -->

# cirque

[![Crates.io](https://img.shields.io/crates/v/cirque.svg)](https://crates.io/crates/cirque)
[![Docs.rs](https://docs.rs/cirque/badge.svg)](https://docs.rs/cirque)
[![Deps.rs](https://deps.rs/repo/github/uklotzde/cirque/status.svg)](https://deps.rs/repo/github/uklotzde/cirque)
[![Security audit](https://github.com/uklotzde/cirque/actions/workflows/security-audit.yaml/badge.svg)](https://github.com/uklotzde/cirque/actions/workflows/security-audit.yaml)
[![Continuous integration](https://github.com/uklotzde/cirque/actions/workflows/continuous-integration.yaml/badge.svg)](https://github.com/uklotzde/cirque/actions/workflows/continuous-integration.yaml)
[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://opensource.org/licenses/MPL-2.0)

Unidirectional, circular, lock/wait-free SPSC queue with unbounded capacity.

Handy for receiving and processing messages in a realtime context.
The messages could be channelled through the realtime context without
deallocating any memory there. Instead they are returned back to the
producer that either recycles or drops them.

## License

Licensed under the Mozilla Public License 2.0 (MPL-2.0) (see [MPL-2.0.txt](LICENSES/MPL-2.0.txt) or <https://www.mozilla.org/MPL/2.0/>).

Permissions of this copyleft license are conditioned on making available source code of licensed files and modifications of those files under the same license (or in certain cases, one of the GNU licenses). Copyright and license notices must be preserved. Contributors provide an express grant of patent rights. However, a larger work using the licensed work may be distributed under different terms and without source code for files added in the larger work.

### Contribution

Any contribution intentionally submitted for inclusion in the work by you shall be licensed under the Mozilla Public License 2.0 (MPL-2.0).

It is required to add the following header with the corresponding [SPDX short identifier](https://spdx.dev/ids/) to the top of each file:

```rust
// SPDX-License-Identifier: MPL-2.0
```

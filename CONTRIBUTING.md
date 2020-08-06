# Contributing to Node-bindgen

Thank you for contributing. No matter how large or small, contributions are always welcome. Before contributing, please read [Code of Conduct](CODE-OF-CONDUCT.md)

#### Table Of Contents

[Assumptions](#assumptions)

[Ask a Question](#ask-a-question)

[Getting Started](#getting-started)

[Contributing](#contributing)

## Assumptions
This project uses v5 of Node N-API. Please see following [compatibility](https://nodejs.org/api/n-api.html#n_api_n_api_version_matrix) matrix.

Familiarity with
- [Rust](https://www.rust-lang.org)
- [Node.js](https://nodejs.org/en/docs/)

Currently, node-bindgen supports the following platforms:

- Linux
- MacOs
- Windows


## Ask a Question

Please open an Issue on GitHub with the label `question`.

## Getting Started

- Please follow [README](https://github.com/infinyon/node-bindgen/blob/master/README.md) for installation instructions.
## Contributing

### Report a Bug

To report a bug, open an issue on GitHub with the label `bug`. Please ensure the issue has not already been reported.

### Suggest an Enhancement

To suggest an enhancement, please create an issue on GitHub with the label `enhancement`.

### Creating pull request

- Fork the `node-bindgen` repository to your GitHub Account.

- Create a branch, submit a PR when your changes are tested and ready for review

If you’d like to implement a new feature, please consider creating a `feature request` issue first to start a discussion about the feature.

### Releasing New Versions

By default, when `master` branch is merged into `release` branch, this automatically will bump the version of the package, tag the new version, build the binary for release and publish the crate to crates.io.

#### Versioning

when opening a PR to merge `master` to `release`, ensure the commit message includes `#major`, `#minor` or `#patch` to handle the versioning correctly.

### License

This project is licensed under the [Apache license](LICENSE-APACHE). Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Fluvio by you, shall be licensed as Apache, without any additional terms or conditions.




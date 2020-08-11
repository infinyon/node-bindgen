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

When a tagged branch with `v*` is pushed, a new release with that reference will be created and all crates in the workspace will be automatically published to crates.io. See the note below about version management.

#### Versioning

Version control is handled by the CI workflow using `cargo cvm -x` to check against the `master` or target branch. If the version has not been updated, the CI will error with a message of which crate(s) have outdated versions. If a version is outdated, the developer can run `cargo cvm -f -s [`major`, `minor` or `patch`]` to automatically bump the crate's version. 

> NOTE: If you run this in the workspace root, it will update all workspace crate versions. If this is not desired, run the command in the crate directory.

### License

This project is licensed under the [Apache license](LICENSE-APACHE). Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Fluvio by you, shall be licensed as Apache, without any additional terms or conditions.




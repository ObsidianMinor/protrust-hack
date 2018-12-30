## Development
Library changes can be done with a nightly Rust installation.

Code-gen changes are more difficult and require some more setup.

### Setup
In all setups you need [cargo-make](https://sagiegurari.github.io/cargo-make/) which can be installed by running `cargo install --force cargo-make`

Windows and Linux have some other dependencies listed below

#### Windows
 * PowerShell 5 or above

#### Linux
 * cURL
 * unzip

To make code-gen development as painless as possible, several cargo-make tasks have been included in the Makefile.toml at the root of this repository.

### Standard workflow
#### Tasks
##### build-gen
`build-gen` is a combination task, it runs the `build` task to build the crate and then the `gen` task to run code regeneration.

##### gen
The `gen` task performs checks to confirm protoc is properly installed, moves existing generated code to ".old" folders, and regenerates all the included code.

##### gen-revert
If you're encountering any issues with the current code generation and would like to return to the previous code, `gen-revert` reverts the previous code generation by deleting the existing code and moving the ".old" folders back to their original place.

##### gen-commit
Any time you want to use the current code generation for another run, run the `gen-commit` task to remove the existing ".old" folders so the next `gen` goes smoothly

##### gen-git-add
It's hard staging all the generated code in git. So `gen-git-add` does that for you
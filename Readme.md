# Gitlab Branch Gen

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Crates.io](https://img.shields.io/crates/v/gitlab-branch-gen)
![GitHub stars](https://img.shields.io/github/stars/stefhol/gitlab-branch-gen)

## Description

`gitlab-branch-gen` streamlines the process of creating a new branch in several GitLab projects at once. You can specify repositories and credentials via a configuration file or command-line arguments, making repetitive tasks effortless.

## Installation

### Release Page Binaries are included

### From Crates.io
```bash
cargo install gitlab-branch-gen
```

### From source
```bash
git clone https://github.com/stefhol/gitlab-branch-gen.git
cd gitlab-branch-gen
cargo install --path .
```

## Configuration

The tool can be configured using a file located at `~/.config/branch-gen/config.yml`.

**Example `config.yml`:**
```yaml
gitlab_url: "https://gitlab.com"
gitlab_api_key: "your_personal_access_token"
repos:
  - "stefhol/test"
  - "stefhol/1234"
  - "inkscape/inkscape"
```

*   **`gitlab_api_key`**: Your GitLab Personal Access Token with `api` scope.
*   **`gitlab_url`**: The base URL of your GitLab instance.
*   **`repos`**: A list of repositories formatted as `namespace/project`.

Arguments passed on the command line will override the values in the configuration file. You can write the provided command-line arguments to the config file by using the `--update-config` flag.

## Usage

### Help

```text
Creates a defined branch in multiple gitlab repos. Config can be placed in `~/.config/branch-gen/config.yml`.

Usage: gitlab-branch-gen [OPTIONS] [BRANCH_NAME]

Arguments:
  [BRANCH_NAME]  The Branch name to create

Options:
      --repos <REPOS>                    The List of repos to change
      --gitlab-api-key <GITLAB_API_KEY>  The Api Key from gitlab
      --gitlab-url <GITLAB_URL>          The gitlab instance to use
  -u, --update-config                    If provided writes to the config
  -c, --config <FILE>                    Overwrites the default config file location
  -h, --help                             Print help
  -V, --version                          Print version
```

### Example Run

This example will attempt to create the `test` branch in the configured repositories.

```bash
gitlab-branch-gen test
```

**Output:**
```text
Branch to create: test
Gitlab Instance: https://gitlab.com
Repositories to change:
- stefhol/test
- stefhol/1234
- stefhol/release
- inkscape/inkscape
Gathering plan on creating branch test
https://gitlab.com/stefhol/test ⚠️ Branch exists ⚠️
stefhol/1234: Skipped Failure on retrieving information
stefhol/release: Skipped Failure on retrieving information
https://gitlab.com/inkscape/inkscape ✅


Execute? Type y to confirm
y


stefhol/test: skipped
stefhol/1234: skipped
stefhol/release: skipped
inkscape/inkscape: https://gitlab.com/inkscape/inkscape error ⛔
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

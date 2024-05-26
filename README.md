# nixpacks-python

`nixpacks-python` is a Python library for building source images using [Nixpacks](https://github.com/railwayapp/nixpacks), a versatile source builder. 

## Installation

### Requirements

- Rust toolchain (with Cargo)
- Python 3.x

### Building and Installation

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/your_username/nixpacks-python.git
    ```

2. **Navigate to the Project Directory:**

    ```bash
    cd nixpacks-python
    ```

3. **Install with Maturin:**

    ```bash
    maturin develop
    ```

    This command will build the Rust crate and make the Python module available for development use.

## Usage

### Importing the Module

```python
import nixpacks_python
```

### Detecting Build Providers

```python
path = "/path/to/your/project"
env_vars = ["VAR1=value1", "VAR2=value2"]
config_file = "config.toml"

providers = nixpacks_python.detect(path, env_vars, config_file)
print(providers)
```

### Creating Build Plans

```python
path = "/path/to/your/project"
env_vars = ["VAR1=value1", "VAR2=value2"]
json_plan = '{"phases": [...]}'  # JSON representation of a build plan
install_cmds = ["npm install", "pip install -r requirements.txt"]
build_cmds = ["make build"]
start_cmd = "python app.py"
apt_pkgs = ["build-essential"]
nix_pkgs = ["python3"]

plan = nixpacks_python.plan(path, env_vars, json_plan, install_cmds, build_cmds, start_cmd, apt_pkgs, nix_pkgs)
print(plan)
```

### Building Docker Images

```python
path = "/path/to/your/project"
name = "my_image"
out_dir = "/output"
print_dockerfile = True
tags = ["latest", "v1.0"]
labels = ["author=John Doe", "version=1.0"]
quiet = False
cache_key = "my_cache_key"
no_cache = False
inline_cache = True
cache_from = "previous_image:latest"
platform = ["linux/amd64"]
current_dir = True
no_error_without_start = False
incremental_cache_image = "incremental_cache:latest"
cpu_quota = "2"
memory = "4G"
verbose = False
docker_host = "tcp://localhost:2375"
docker_tls_verify = "/path/to/cert.pem"
env_vars = ["VAR1=value1", "VAR2=value2"]
config_file = "config.toml"

nixpacks_python.build(path, name, out_dir, print_dockerfile, tags, labels, quiet, cache_key, no_cache, inline_cache, cache_from, platform, current_dir, no_error_without_start, incremental_cache_image, cpu_quota, memory, verbose, docker_host, docker_tls_verify, env_vars, config_file)
```

## Contribution

Contributions to `nixpacks-python` are welcome! If you have any ideas for improvements or new features, feel free to open an issue or submit a pull request.

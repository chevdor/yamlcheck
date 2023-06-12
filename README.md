# yamlcheck

A CLI to check a YAML file based on a json schema.

## Install

### Using Cargo

    cargo install yamlcheck

### Using Podman

Well.. nothing to install. An alias at most:
```
alias yamlcheck='podman run --rm -t docker.io/chevdor/yamlcheck'
```

## Usage

    SCHEMA=whatever.json
    YAML=foo.yaml
    yamlcheck check -s $SCHEMA --file $YAML

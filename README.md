# yamlcheck

A CLI to check a YAML file based on a json schema.

## Install

    cargo install yamlcheck

## Usage

    SCHEMA=whatever.json
    YAML=foo.yaml
    yamlcheck check -s $SCHEMA --file $YAML

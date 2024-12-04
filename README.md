# yamlcheck

A CLI based on [valico](https://github.com/s-panferov/valico) to check a YAML file using a json schema.

## Install

### Using Cargo

    cargo install yamlcheck

### Using Podman/Docker

Well.. nothing to install. An alias at most:
```
alias yamlcheck='podman run --rm -t docker.io/chevdor/yamlcheck'
```

## Usage

`samples/01/file.yaml`:
```
name: Bob
age: 30
comment: |
  This is a comment.
  It can be multiple lines.
```

`samples/01/schema.json`:
```
{
    "$schema": "https://json-schema.org/draft/2020-12/schema#",
    "title": "Sample Schema",
    "description": "Sample Schema",
    "type": "object",
    "additionalProperties": true,
    "properties": {
        "name": {
            "type": "string",
            "description": "Name"
        },
        "age": {
            "description": "Some numbers",
            "type": "number"
        },
        "comment": {
            "description": "Optional: Some comment",
            "type": "string"
        }
    },
    "required": [
        "name",
        "age"
    ]
}
```

Sample run:

    SCHEMA=samples/01/schema.json
    YAML=samples/01/file.yaml
    yamlcheck check -s $SCHEMA --file $YAML

Output:

    valid          : true
    strictly valid : true

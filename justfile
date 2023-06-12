export VERSION := `toml get Cargo.toml package.version | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

build:
  echo "Building version $VERSION"
  podman build -t yamlcheck -t docker.io/chevdor/yamlcheck -t "docker.io/chevdor/yamlcheck:$VERSION" .
  podman images | grep yamlcheck

docker_push: build
  podman push docker.io/chevdor/yamlcheck docker.io/chevdor/yamlcheck:$VERSION

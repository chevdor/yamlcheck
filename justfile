ENGINE := "${ENGINE:-docker}"
DOCKER_USER := "${DOCKER_USER:-chevdor}"
IMAGE := "yamlcheck"
REGISTRY := "${REGISTRY:-docker.io}"
export VERSION := `toml get Cargo.toml package.version | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

build:
  echo "Building version $VERSION"
  {{ENGINE}} build \
    -t yamlcheck \
    -t {{REGISTRY}}/{{DOCKER_USER}}/{{IMAGE}} \
    -t "{{REGISTRY}}/{{DOCKER_USER}}/{{IMAGE}}:$VERSION" \
    .

  {{ENGINE}} images | grep yamlcheck

docker_push: build
  {{ENGINE}} push {{REGISTRY}}/{{DOCKER_USER}}/{{IMAGE}}
  {{ENGINE}} push {{REGISTRY}}/{{DOCKER_USER}}/{{IMAGE}}:$VERSION

IMAGE_NIXOS := "freedesktop-sound-nixos"
IMAGE_UBUNTU := "freedesktop-sound-ubuntu"
DOCKERFILE_NIXOS := "./images/Dockerfile.nixos"
DOCKERFILE_UBUNTU := "./images/Dockerfile.ubuntu"

default: 
    @just --list

build: build-nixos build-ubuntu

build-nixos:
    docker build --progress=plain -f {{DOCKERFILE_NIXOS}} -t {{IMAGE_NIXOS}} .

build-ubuntu:
    docker build --progress=plain -f {{DOCKERFILE_UBUNTU}} -t {{IMAGE_UBUNTU}} .

test-nixos: build-nixos
    docker run {{IMAGE_NIXOS}} 

test-ubuntu: build-ubuntu
    docker run {{IMAGE_UBUNTU}}

test: test-nixos test-ubuntu

clean:
    docker rm -f {{IMAGE_NIXOS}} {{IMAGE_UBUNTU}} || true

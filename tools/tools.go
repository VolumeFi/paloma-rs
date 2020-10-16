// package tool pins a number of Go dependencies that we use. Go builds really fast,
// so wherever we can, we just build from source rather than pulling in third party binaries.
package main

//noinspection ALL
import (
	_ "github.com/bufbuild/buf/cmd/buf"
	_ "google.golang.org/grpc/cmd/protoc-gen-go-grpc"
	_ "google.golang.org/protobuf/cmd/protoc-gen-go"
)

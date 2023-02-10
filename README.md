# Binding SCS to go and rust

1. Must install intel mkl to `/opt/intel/oneapi/mkl/2023.0.0`.
1. Reasonably new gcc.
1. Bazelisk (or bazel)
  ```shell
  go install github.com/bazelbuild/bazelisk@latest
  ```
1. clone this repo.

  ```bash
  # for go
  bazelisk run //exp-go
  # for rust
  bazelisk run //exp-rust
  ```

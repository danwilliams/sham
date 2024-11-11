# Sham

![Rust](https://img.shields.io/badge/Rust-1.81%2B-b7410e?style=flat&logo=rust&logoColor=white&labelColor=b7410e)
[![Crate version](https://img.shields.io/crates/v/sham?style=flat)](https://crates.io/crates/sham)
[![CI](https://img.shields.io/github/actions/workflow/status/danwilliams/sham/ci.yml?style=flat&logo=github&logoColor=white&label=build%2Ftest)](https://github.com/danwilliams/sham/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/docsrs/sham?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/crate/sham/latest)
![License](https://img.shields.io/github/license/danwilliams/sham?style=flat)

> /ʃam/\
> *noun*      - a thing that is not what it is purported to be.\
> *adjective* - not genuine; fake or false.\
> *verb*      - falsely present something as the truth.

Sham is a collection of useful mocks and fakes for testing Rust code. The
primary purpose is to be able to swap out a genuine implementation and
substitute it with a sham one, in order to achieve deterministic testing
without side effects.

This is particularly useful for testing code that usually performs a particular
operation that is either expensive, slow, or has side effects that are
undesirable in a test environment, such as sending network requests. In these
cases, a sham implementation can be used to simulate the real one, without
actually performing the operation.

The modules provided are:

  - [`reqwest`](#reqwest)
  - [`std_process`](#std_process)

Note, each module is behind a feature flag, in order to keep the crate size down
for those who don't need all the functionality.


## `reqwest`

The [`reqwest`](https://docs.rs/sham/latest/sham/reqwest/index.html) module
provides mocks for the [Reqwest](https://docs.rs/reqwest/) crate, which is a
popular HTTP client for Rust.


## `std_process`

The [`std_process`](https://docs.rs/sham/latest/sham/std_process/index.html)
module provides mocks for the [Rust standard library's process module](https://doc.rust-lang.org/std/process/),
mainly and most notably [`Command`](https://doc.rust-lang.org/std/process/struct.Command.html).



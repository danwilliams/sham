# Sham

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



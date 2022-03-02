# Echo Trace

Upstream [`echo`]() example ported to a standalone Crate.

The `Cargo.toml` shows a minimal set of dependencies to build a traceable
asynchronous server.

This file also shows the use of the Cargo [`patch`]() stanza to, well, patch-in
your implementation of the proc-macro attributes - as you require.
Cargo's `patch` is a poorman's (compile-time) plugin system.

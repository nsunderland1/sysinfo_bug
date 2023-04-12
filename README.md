Reproduction for a bug in the `sysinfo` crate.

To run this, you need two users: one for the process to initially run under, and another for it to call `seteuid` with.
In my own testing, I just used root and `$(whoami)` for this.

```
$ cargo build
$ sudo ./target/debug/sysinfo_bug $(id -u $(whoami))
```

This should fail an assertion because `sysinfo` will incorrectly report the effective user ID as `0` (root) (see `main.rs` for details)
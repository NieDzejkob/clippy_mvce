```
Checking clippy_mvce v0.1.0 (/home/kuba/dev/clippy_mvce)
error: useless lint attribute
 --> src/other_file.rs:3:1
  |
3 | #[derive(FromPrimitive, PartialEq, Debug)]
  | ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[deny(clippy::useless_attribute)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.212/index.html#useless_attribute

error: aborting due to previous error

error: Could not compile `clippy_mvce`.

To learn more, run the command again with --verbose.
```

Removing `FromPrimitive` from the attribute generates compile errors, obviously.
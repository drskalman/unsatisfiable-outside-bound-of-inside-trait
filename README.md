running 

```
cargo test
```

on crate1 results in 
```
error[E0277]: the trait bound `Struct1: crate1::Trait1` is not satisfied
  --> crate1/src/lib.rs:16:23
   |
16 |         let my_enum : Enum1<Struct1>;
   |                       ^^^^^^^^^^^^^^ the trait `crate1::Trait1` is not implemented for `Struct1`
   |
   = help: the trait `crate1::Trait1` is implemented for `crate1::Struct1`
note: required by a bound in `Enum1`
  --> /home/klaymen/wormhole/code/playground/rust-playground/unsatisfiable-outside-bound-of-inside-trait/crate2/src/lib.rs:4:53
   |
4  | pub enum Enum1<GenericStruct> where GenericStruct : Trait1 {
   |                                                     ^^^^^^ required by this bound in `Enum1`

For more information about this error, try `rustc --explain E0277`.
```

which is not true.

```
$ rustc --version
rustc 1.75.0 (82e1608df 2023-12-21)
```

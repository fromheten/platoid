# `platoid`

`platoid` takes any data on stdin, and returns it's platonic ID-number.

## Platonic ID-number

The true ID of any data is it's hash. With this ID, the idea is no longer bound to place by the flesh.

The platonic id is an identifier, version number, and alphanumerically encoded 256 bit Sha3 hash of the input.

## Example

```
$ echo "Λόγος" | ./target/debug/platoid
plato0:VBheL9Z2dgMiTFs3JQEh9VmEB2n9jGOE5vk2thjDvmZ
```

# Building and installing

```
$ cargo build --release && mv ./target/release/platoid /where/i/keep/my/binaries
```

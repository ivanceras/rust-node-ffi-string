
Most of rust-ffi to nodejs example is returning an integer. This is not so practical at all, since we want to call a feature complete api.

By feature complete I mean JSON as both argument and return


## Install node ffi

```sh
npm install ffi --save
```

## Build and run!

```sh
cargo build --release
node string.js
````

### Rust/Scala WASM Interop

Invoking Rust functions via Golem workers from Scala code. Currently non-functional as Scala bindgen does not support imports.

- https://github.com/golemcloud/golem-scalajs-wit-bindgen
- https://github.com/golemcloud/golem-scalajs-wit-bindgen/issues/5


```
git clone git@github.com:golemcloud/sbt-wasm-component.git
cd sbt-wasm-component
sbt +publishLocal
```

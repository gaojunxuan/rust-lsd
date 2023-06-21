# rust-lsd
Rust implementation of linear-time direct superbubble detector

The implementation uses the algorithm first proposed in [Direct Superbubble Detection by Gärtner et al (2019)](https://www.mdpi.com/1999-4893/12/4/81). 
Reference implementation in Python is given at https://github.com/Fabianexe/Superbubble

# Build
To build, run
```
cargo build
```
and then
```
target/debug/direct_superbubble <path to an edge list>
```
The input should be a directed graph represented as an edge list where every line contains a pair of vertices separated by a white space.

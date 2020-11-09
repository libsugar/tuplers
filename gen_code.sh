cd ./code_gen/
cargo build --release
cd ..
./target/release/code_gen

rustfmt ./tuples/src/gen/cloned.rs
rustfmt ./tuples/src/gen/combin.rs
rustfmt ./tuples/src/gen/flatten.rs
rustfmt ./tuples/src/gen/transpose.rs
rustfmt ./tuples/src/gen/tuple_alias.rs
rustfmt ./tuples/src/gen/tuple_as.rs
rustfmt ./tuples/src/gen/tuple_impl.rs
rustfmt ./tuples/src/gen/tuple_iter.rs
rustfmt ./tuples/src/gen/tuple_map.rs
rustfmt ./tuples/src/gen/tuple_n.rs

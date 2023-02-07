Set-Location .\code_gen\
cargo build --release
Set-Location ..
.\target\release\code_gen.exe

rustfmt .\tuples\src\gen\cloned.rs
rustfmt .\tuples\src\gen\combin.rs
rustfmt .\tuples\src\gen\split_by.rs
rustfmt .\tuples\src\gen\split_to_tuple_by.rs
rustfmt .\tuples\src\gen\split_at.rs
rustfmt .\tuples\src\gen\split_to_tuple_at.rs
rustfmt .\tuples\src\gen\flatten.rs
rustfmt .\tuples\src\gen\transpose.rs
rustfmt .\tuples\src\gen\tuple_alias.rs
rustfmt .\tuples\src\gen\tuple_as.rs
rustfmt .\tuples\src\gen\tuple_call.rs
rustfmt .\tuples\src\gen\tuple_impl.rs
rustfmt .\tuples\src\gen\tuple_iter.rs
rustfmt .\tuples\src\gen\tuple_map.rs
rustfmt .\tuples\src\gen\tuple_n.rs

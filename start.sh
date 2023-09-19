{
cargo watch -q -c -w src/ -x run &
cargo watch -q -c -w src/tests/ -x "test -q quick_test -- --nocapture"
}
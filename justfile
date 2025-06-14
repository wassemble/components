bindings component:
    cd crates/{{component}} && wkg wit fetch && cargo component bindings

build component: (bindings component)
    cargo component build -p {{component}} --release

check:
    cargo +nightly fmt
    cargo check
    cargo clippy --allow-dirty --fix
    cargo machete
    cargo sort-derives

install: 
    cargo install --locked cargo-component
    cargo install --locked cargo-machete
    cargo install --locked cargo-sort-derives
    cargo install --locked cargo-watch
    cargo install --locked wkg

new component:
    cargo component new --editor none --lib --namespace wassemble crates/{{component}}

publish component: (build component)
    @safe_component=$(echo {{component}} | tr '-' '_') && \
    wkg oci push ghcr.io/wassemble/{{component}}:latest target/wasm32-wasip1/release/$safe_component.wasm

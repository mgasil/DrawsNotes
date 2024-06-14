<picture>
<img src="https://raw.githubusercontent.com/MoonKraken/DrawsNotes/main/demo.gif" />
</picture>

# DrawNotes
More details about the creation of this project can be found in [this video](https://youtu.be/Pr6T0Phjvgc).

A very simple note-taking app built with an all-Rust stack: Leptos for the frontend and backend (Axum under the hood on the backend), and SurrealDB as the database.

To run:

`cargo leptos serve`

# [Leptos](https://book.leptos.dev)
To install:

1. `rustup target add wasm32-unknown-unknown`
1. `rustup toolchain install nightly --allow-downgrade`
1. `rustup default nightly` - setup nightly as default, or you can use rust-toolchain file later on`
1. `cargo install cargo-leptos`

# [SurrealDB](https://surrealdb.com/docs/surrealdb/installation)

To run:

`surreal start memory -A`

# [Tailwind CSS](https://tailwindcss.com/docs/installation)

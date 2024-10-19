# Releasing

The Rust port is published to [crates.io](https://crates.io/).

Run these commands in the `rs` directory:

1. `git checkout main` to get on the main branch
2. `git pull` to get the latest from the remote
3. `git status` to check if there are any dirty files
4. If no crates.io token on hand, generate one in the web app
5. `cargo login`
6. Increment the version in `Cargo.toml`
7. `git add Cargo.toml Cargo.lock` to stage the version change
8. `git commit -m "chore(rs): release X.X.X"` to commit the verison bump
9. `cargo publish --dry-run` to see what will happen on publish
10. `cargo publish` to publish the package
11. `git push` to push the release commit

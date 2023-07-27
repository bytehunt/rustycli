# Creating a Release

[GitHub](https://github.com/pwnwriter/rustycli/releases) and [crates.io](https://crates.io/crates/rustycli) releases are automated via [GitHub actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Run the [release script](./release.sh): `./release.sh v[X.Y.Z]` (requires [rustycli](https://github.com/pwnwriter/rustycli) for changelog generation)
2. Push the changes: `git push`
3. Check if [Continuous Integration](https://github.com/pwnwriter/rustycli/actions) workflow is completed successfully.
4. Push the tags: `git push --tags`
5. Wait for [Continuous Deployment](https://github.com/pwnwriter/rustycli/actions) workflow to finish.

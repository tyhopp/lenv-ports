# Releasing

The JS port is published to [npm](https://www.npmjs.com/).

1. `git checkout main` to get on the main branch
2. `git pull` to get the latest from the remote
3. `git status` to check if there are any dirty files
4. `npm login` to log into npm via the browser
5. `npm version major|minor|patch` to bump the package version
6. `npm publish --dry-run` to see what publishing will do
7. `npm publish` to publish the package
8. `git commit -m "chore(js): release X.X.X"` to commit the verison bump
9. `git push` to push the release commit

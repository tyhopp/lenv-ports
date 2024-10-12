# Releasing

The Python port is published to [PyPI](https://pypi.org/).

Run these commands in the `py` directory:

1. `git checkout main` to get on the main branch
2. `git pull` to get the latest from the remote
3. `git status` to check if there are any dirty files
4. Increment the version in `pyproject.toml`
5. `uv sync` to update the lockfile
6. `uv build` to build the wheel and tar binaries
7. If no PyPI token on hand, generate one in the web app
8. `uv publish --token [your-token]` to publish the package
9. `git add pyproject.toml uv.lock` to stage the version change
10. `git commit -m "chore(py): release X.X.X"` to commit the verison bump
11. `git push` to push the release commit

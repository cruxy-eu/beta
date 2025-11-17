# Post Comment

This is an action for posting a comment on a GitHub pull request.

## Beta

```yaml
name: Post Comment

on:
  pull_request:

jobs:
  post-comment:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout code
        uses: actions/checkout@v2

      - name: Post comment
        uses: actions/post-comment@v1
        with:
          number: ${{ github.event.pull_request.number }}
          token: ${{ secrets.GITHUB_TOKEN }}
          body: "# Supports markdown"
```

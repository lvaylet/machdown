#!/usr/bin/env bash

# Build full context with issues + last 5 commits + prompt,
# and feed it to Jetski for a headless (i.e. non-interactive) session.

issues=$(cat issues/*.md 2>/dev/null || echo "No issues found")
commits=$(git log -n 5 --format="%H%n%ad%n%B---" --date=short 2>/dev/null || echo "No commits found")
prompt=$(cat ralph/prompt.md)

cat << EOM | /google/bin/releases/jetski-devs/tools/cli 
Previous commits:

${commits}

---

Issues:

${issues}

---

Prompt:

${prompt}
EOM

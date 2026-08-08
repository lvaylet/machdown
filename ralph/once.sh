#!/usr/bin/env bash

# Build full context with issues + last 5 commits + prompt,
# and feed it to Jetski for a headless (i.e. non-interactive) session.
#
issues=$(cat issues/*.md 2>/dev/null || echo "No issues found")
commits=$(git log -n 5 --format="%H%n%ad%n%B---" --date=short 2>/dev/null || echo "No commits found")
ralph_prompt=$(cat ralph/prompt.md)

# WARNING Avoid using `--dangerously-skip-permissions` in sensitive production
# environments without adequate sandboxing.
{
  echo Previous commits:
  echo "${commits}"
  echo ---
  echo Issues:
  echo "${issues}"
  echo ---
  echo Prompt:
  echo "${ralph_prompt}"
} | /google/bin/releases/jetski-devs/tools/cli --dangerously-skip-permissions

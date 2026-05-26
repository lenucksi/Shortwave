#!/usr/bin/env bash
set -euo pipefail

HAS_ERROR=0
CACHE_DIR="${TMPDIR:-/tmp}/action-sha-cache"
mkdir -p "$CACHE_DIR"

cache_file() { printf '%s' "$1" | sha256sum | cut -c1-16; }

get_refs() {
  local repo="$1" cf
  cf="$CACHE_DIR/$(cache_file "$repo")"
  if [[ -f "$cf" ]]; then cat "$cf"; return; fi
  printf '\e[34m*\e[0m fetching refs for %s ...\n' "$repo" >&2
  git ls-remote "https://github.com/$repo.git" > "$cf" 2>/dev/null || { rm -f "$cf"; return 1; }
  cat "$cf"
}

latest_tag() {
  awk '/refs\/tags\/v/ && !/refs\/tags\/.*\^{}/ {
    t = $2; sub("refs/tags/", "", t); print t, $1
  }' "$1" | sort -k1 -V | tail -1
}

verify() {
  local file="$1" repo="$2" sha="$3" tag="$4" refs
  tag="${tag#\# }"  # strip "# " comment prefix
  refs="$(get_refs "$repo")" || {
    printf '  \e[31mFAIL\e[0m %s: cannot fetch %s\n' "$file" "$repo"
    HAS_ERROR=1; return
  }
  if ! grep -q "$sha" <<< "$refs"; then
    printf '  \e[31mFAIL\e[0m %s: SHA %s NOT FOUND in %s\n' "$file" "$sha" "$repo"
    local lv ls
    lv="$(latest_tag <(echo "$refs"))"; ls="${lv#* }"; lv="${lv% *}"
    [[ -n "$lv" ]] && printf '    \e[33mWARN\e[0m latest: %s @ %s\n' "$lv" "$ls"
    HAS_ERROR=1; return
  fi
  if [[ -n "$tag" ]]; then
    local ts
    ts="$(awk -v t="refs/tags/$tag" '$2 == t {print $1; exit}' <<< "$refs")"
    if [[ -z "$ts" ]]; then
      printf '  \e[33mWARN\e[0m %s: tag %s not found\n' "$file" "$tag"
    elif [[ "$ts" != "$sha" ]]; then
      printf '  \e[31mFAIL\e[0m %s: SHA mismatch for %s — expected %s, got %s\n' "$file" "$tag" "$ts" "$sha"
      HAS_ERROR=1; return
    else
      printf '  \e[32mOK\e[0m   %s: %s @ %s (%s)\n' "$file" "$repo" "$tag" "$sha"
    fi
  else
    printf '  \e[32mOK\e[0m   %s: %s @ %s\n' "$file" "$repo" "$sha"
  fi
  local latest
  latest="$(latest_tag <(echo "$refs"))"
  local lv="${latest% *}" ls="${latest#* }"
  if [[ -n "$tag" && -n "$lv" && "$tag" != "$lv" ]]; then
    printf '  \e[33mWARN\e[0m %s: %s %s outdated — latest %s (%s)\n' "$file" "$repo" "$tag" "$lv" "$ls"
  fi
}

echo "=== GitHub Actions SHA verification ==="
echo ""

for f in .github/workflows/*.yml; do
  [[ -f "$f" ]] || continue
  while IFS= read -r line; do
    line="${line#uses: }"
    r="${line%%@*}"; rest="${line#*@}"
    s="${rest%% *}"; t="${rest#* }"
    [[ "$s" == "$t" ]] && t=""
    verify "$f" "$r" "$s" "$t"
  done < <(grep -oE 'uses: [a-zA-Z0-9_.-]+/[a-zA-Z0-9_.-]+@[a-f0-9]{40}( # v[0-9][^ ]*)?' "$f" 2>/dev/null || true)
done

printf '\n'
[[ $HAS_ERROR -eq 1 ]] && { echo "FAILED."; exit 1; }
echo "All SHAs verified."

#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
cd "${ROOT_DIR}"

echo "=== Markdown Relative Link Check ==="

broken=0

for md_file in $(find . -name "*.md" -not -path "./target/*" -not -path "./.git/*"); do
    md_dir=$(dirname "$md_file")
    # Extract relative markdown links
    links=$(grep -oP '\[[^]]*\]\(([^)]+)\)' "$md_file" 2>/dev/null | grep -oP '(?<=\().+(?=\))' || true)
    for link in $links; do
        # Skip absolute URLs
        case "$link" in
            http://*|https://*|mailto:*|#*) continue ;;
        esac
        # Remove anchor for file check
        target="${link%%#*}"
        if [ -n "$target" ] && [ ! -e "$md_dir/$target" ]; then
            echo "[BROKEN] $md_file -> $link"
            broken=$((broken + 1))
        fi
    done
done

echo ""
if [ "$broken" -eq 0 ]; then
    echo "✅ All relative links valid."
else
    echo "❌ $broken broken links found."
fi

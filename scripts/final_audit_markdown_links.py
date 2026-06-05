#!/usr/bin/env python3
"""Final markdown link audit for rust-from-python tutorial.
Checks all internal (relative) links in markdown files.
Uses only Python 3 standard library.
"""

import re
import sys
import urllib.parse
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
REPORT_DIR = ROOT / "final_audit_reports"
REPORT_DIR.mkdir(exist_ok=True)

# Find inline links: [text](url) and [text](url#anchor)
LINK_PATTERN = re.compile(r'\[([^\]]*)\]\(([^)]+)\)')

def github_slugify(heading: str) -> str:
    """Generate GitHub-style heading anchor from markdown heading text.
    Matches GitHub's actual behavior for Chinese content.
    """
    text = heading.strip()
    # Remove leading #s and whitespace
    text = re.sub(r'^#+\s*', '', text)
    # Remove backticks
    text = text.replace('`', '')
    # Remove individual punctuation characters (both ASCII and CJK)
    # NOTE: underscore '_' is NOT removed — GitHub preserves it in anchors
    punct = r'!"#$%&\'()*+,./:;<=>?@[\\\]^{|}~，。！？；：""''…—～【】《》（）、'
    for ch in punct:
        text = text.replace(ch, '')
    text = text.strip()
    # Lowercase
    text = text.lower()
    # Replace each space with a single dash
    text = re.sub(r'\s+', '-', text.strip())
    # Remove consecutive dashes (from space groups if any remain)
    text = re.sub(r'-{2,}', '-', text)
    return text.strip('-')

def check_link(link_path: str, source_file: Path) -> tuple:
    """Check if an internal link is valid. Returns (valid, reason)."""
    # Skip external URLs
    if link_path.startswith(('http://', 'https://', 'mailto:')):
        return True, "external"

    # Parse URL to separate path from anchor
    if '#' in link_path:
        file_part, anchor = link_path.split('#', 1)
    else:
        file_part = link_path
        anchor = None

    # Empty file part with anchor = same-file anchor
    if not file_part:
        target_file = source_file
    else:
        # Resolve relative path
        decoded = urllib.parse.unquote(file_part)
        target_file = (source_file.parent / decoded).resolve()

    # Does the file exist?
    if not file_part:
        # Same-file anchor only
        if anchor:
            return check_anchor(source_file, anchor)
        return True, "same-file no anchor"

    if not target_file.is_file():
        # Could be a directory (link should have /README.md or index)
        if target_file.is_dir():
            # Try README.md in that directory
            if (target_file / "README.md").is_file():
                if anchor:
                    return check_anchor(target_file / "README.md", anchor)
                return True, "directory has README.md"
            return False, f"directory exists but no README.md: {target_file}"

        # Check if it's a directory-style link that should point to a .md file
        if not file_part.endswith('.md') and not file_part.endswith('/'):
            # Try adding .md or /README.md
            md_target = Path(str(target_file) + '.md')
            if md_target.is_file():
                if anchor:
                    return check_anchor(md_target, anchor)
                return True, "auto .md"

            readme_target = target_file / "README.md"
            if readme_target.is_file():
                if anchor:
                    return check_anchor(readme_target, anchor)
                return True, "auto /README.md"

        return False, f"target file not found: {target_file}"

    if anchor:
        return check_anchor(target_file, anchor)

    return True, "file exists"

def check_anchor(target_file: Path, anchor: str) -> tuple:
    """Check if an anchor exists in a markdown file."""
    if not target_file.is_file():
        return False, f"target file for anchor not found: {target_file}"

    content = target_file.read_text(encoding="utf-8")

    # GitHub anchors: slugify each heading
    for line in content.split("\n"):
        stripped = line.strip()
        if stripped.startswith("#"):
            slug = github_slugify(stripped)
            if slug == anchor:
                return True, "anchor found"

    # Also check for explicit <a name=...> or id=... anchors
    if f'id="{anchor}"' in content or f"id='{anchor}'" in content:
        return True, "explicit id"
    if f'name="{anchor}"' in content or f"name='{anchor}'" in content:
        return True, "explicit name"

    return False, f"anchor not found: #{anchor}"

def main():
    md_files = list(ROOT.glob("chapters/**/*.md")) + \
               list(ROOT.glob("projects/**/*.md")) + \
               [f for f in ROOT.glob("*.md") if f.name != "Cargo.lock"]

    output = []
    output.append("# Final Markdown Link Audit\n")
    output.append(f"**Date**: 2026-06-05\n")
    output.append("")

    total_links = 0
    broken_links = 0
    external_links = 0
    broken_details = []

    for md_file in sorted(md_files):
        rel_path = str(md_file.relative_to(ROOT))
        content = md_file.read_text(encoding="utf-8")

        for m in LINK_PATTERN.finditer(content):
            total_links += 1
            link_text = m.group(1)
            link_url = m.group(2)

            # Get line number
            line_num = content[:m.start()].count('\n') + 1

            if link_url.startswith(('http://', 'https://', 'mailto:')):
                external_links += 1
                continue

            valid, reason = check_link(link_url, md_file)
            if not valid:
                broken_links += 1
                broken_details.append({
                    "file": rel_path,
                    "line": line_num,
                    "text": link_text[:60],
                    "url": link_url[:100],
                    "reason": reason,
                })

    output.append("## Summary\n")
    output.append("| Metric | Count |")
    output.append("|--------|------:|")
    output.append(f"| Total markdown files scanned | {len(md_files)} |")
    output.append(f"| Total internal links | {total_links - external_links} |")
    output.append(f"| External links (not validated) | {external_links} |")
    output.append(f"| Broken internal links | {broken_links} |")
    output.append("")

    if broken_details:
        output.append("## Broken Links\n")
        output.append("| File | Line | Link Text | URL | Reason |")
        output.append("|------|-----:|-----------|-----|--------|")
        for b in broken_details:
            output.append(f"| {b['file']} | {b['line']} | {b['text']} | `{b['url']}` | {b['reason']} |")
        output.append("")
    else:
        output.append("## ✅ All internal links valid\n")

    report = "\n".join(output)
    out_path = REPORT_DIR / "markdown_links.md"
    out_path.write_text(report, encoding="utf-8")
    print(f"Link audit written to {out_path}")
    print(f"  Files scanned: {len(md_files)}")
    print(f"  Internal links: {total_links - external_links}")
    print(f"  External links: {external_links}")
    print(f"  Broken: {broken_links}")

    if broken_links > 0:
        print("ERROR: Broken internal links!")
        sys.exit(1)

    sys.exit(0)

if __name__ == "__main__":
    main()

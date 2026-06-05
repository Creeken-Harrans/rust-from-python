#!/usr/bin/env python3
"""Content quality audit: scans for terminology coverage, inaccurate statements, and background/motivation coverage."""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPORT_DIR = os.path.join(ROOT, "audit_reports")
os.makedirs(REPORT_DIR, exist_ok=True)

# ── 8.1 Core terminology check ────────────────────────────────────────
CORE_TERMS = [
    "Ownership", "Move", "Copy", "Clone", "Borrowing",
    "Reference", "Mutable Reference", "Slice", "Lifetime",
    "Borrow Checker", "Non-Lexical Lifetimes", "RAII",
    "Drop", "Struct", "Enum", "Pattern Matching", "Option",
    "Result", "Generic", "Trait", "Trait Bound",
    "Monomorphization", "Static Dispatch", "Dynamic Dispatch",
    "Trait Object", "Smart Pointer", "Interior Mutability",
    "Reference Counting", "Data Race", "Mutex", "Arc",
    "Rc", "RefCell", "Async", "Concurrency", "Parallelism",
    "Future", "Runtime", "Macro", "FFI", "Unsafe",
    "Workspace", "Package", "Crate", "Module",
]

# English variant -> Chinese variants we check for
TERM_CHINESE = {
    "Ownership": "所有权",
    "Move": "移动",
    "Copy": "复制",
    "Clone": "克隆",
    "Borrowing": "借用",
    "Reference": "引用",
    "Mutable Reference": "可变引用",
    "Slice": "切片",
    "Lifetime": "生命周期",
    "Borrow Checker": "借用检查器",
    "RAII": "RAII",
    "Drop": "Drop",
    "Struct": "结构体",
    "Enum": "枚举",
    "Pattern Matching": "模式匹配",
    "Option": "Option",
    "Result": "Result",
    "Generic": "泛型",
    "Trait": "特征",
    "Trait Bound": "特征约束",
    "Monomorphization": "单态化",
    "Static Dispatch": "静态分派",
    "Dynamic Dispatch": "动态分派",
    "Trait Object": "特征对象",
    "Smart Pointer": "智能指针",
    "Interior Mutability": "内部可变性",
    "Reference Counting": "引用计数",
    "Data Race": "数据竞争",
    "Mutex": "Mutex",
    "Arc": "Arc",
    "Rc": "Rc",
    "RefCell": "RefCell",
    "Async": "异步",
    "Concurrency": "并发",
    "Parallelism": "并行",
    "Future": "Future",
    "Runtime": "运行时",
    "Macro": "宏",
    "FFI": "FFI",
    "Unsafe": "Unsafe",
    "Workspace": "Work空间",
    "Package": "包",
    "Crate": "箱",
    "Module": "模块",
}

# Check which chapter mentions which term
term_first_use = {}
term_chapters = {t: [] for t in CORE_TERMS}

for dp, _, files in os.walk(os.path.join(ROOT, "chapters")):
    for f in files:
        if not f.endswith(".md"):
            continue
        path = os.path.join(dp, f)
        chapter = os.path.basename(dp)
        try:
            with open(path, "r", encoding="utf-8") as fh:
                content = fh.read()
        except Exception:
            continue

        for term in CORE_TERMS:
            if term.lower() in content.lower():
                term_chapters[term].append(chapter)
                if term not in term_first_use:
                    term_first_use[term] = chapter

# Check for bilingual introduction
missing_bilingual = []
for term, ch in term_first_use.items():
    chinese = TERM_CHINESE.get(term, "")
    if not chinese or chinese == term:
        continue
    # Check if the chapter introduces this term with Chinese
    readme = os.path.join(ROOT, "chapters", ch, "README.md")
    try:
        with open(readme, "r", encoding="utf-8") as fh:
            content = fh.read()
        # Simple heuristic: term appears near Chinese term
        if term.lower() in content.lower() and chinese in content:
            pass  # both appear
        elif term.lower() in content.lower():
            missing_bilingual.append(f"{term} first in {ch} but no Chinese '{chinese}' nearby")
    except Exception:
        pass

term_missing = [t for t in CORE_TERMS if not term_chapters[t]]

# ── 8.2 Inaccurate statements search ──────────────────────────────────
INACCURATE_PATTERNS = [
    ("Rust 完全没有运行时开销", "Rust has runtime costs (bounds checks, RefCell, Mutex, Arc)"),
    ("Rust 没有运行时开销", "Rust has runtime costs (bounds checks, RefCell, Mutex, Arc)"),
    ("Rust 完全不需要堆", "Rust uses heap via Box, Vec, String, etc."),
    ("Rust 不使用堆", "Rust uses heap via Box, Vec, String, etc."),
    ("Rust 所有值都在栈上", "Rust has heap-allocated types (Box, Vec, String)"),
    ("Rust 所有权就是引用计数", "Ownership is compile-time, not reference counting"),
    ("Move 就是深拷贝", "Move is ownership transfer, not deep copy"),
    ("Move 等于深拷贝", "Move is ownership transfer, not deep copy"),
    ("Rust Move 等于 C++ std::move", "Rust Move ≠ C++ std::move (different mechanisms)"),
    ("clone 是解决借用问题", "clone() creates a copy; borrowing avoids copy"),
    ("clone 是解决所有权问题的推荐方法", "Borrowing is the idiomatic solution, not clone()"),
    ("Rust 引用就是 C 指针", "Rust references have safety guarantees C pointers lack"),
    ("Rust 引用和 C 指针完全相同", "Rust references have safety guarantees C pointers lack"),
    ("生命周期标注会延长变量寿命", "Lifetime annotations describe relationships, don't extend lifetimes"),
    ("生命周期可以延长变量寿命", "Lifetime annotations describe relationships, don't extend lifetimes"),
    ("'static 就是全局变量", "'static means 'lives for entire program', not necessarily global"),
    ("Option 只是 null", "Option is a sum type with compiler-enforced handling"),
    ("Option 就是 null", "Option is a sum type with compiler-enforced handling"),
    ("Result 就是异常", "Result is a return type encoding success/failure, not exception"),
    ("Trait 就是接口", "Trait is not just an interface (has default impl, associated types, blanket impl)"),
    ("Trait 就是抽象类", "Trait is not just an abstract class (no inheritance, static dispatch possible)"),
    ("Arc 自动保证线程安全", "Arc only shares ownership; Mutex/RwLock needed for mutation safety"),
    ("Arc 自动保证内部数据线程安全", "Arc only shares ownership; Mutex/RwLock needed for mutation safety"),
    ("RefCell 可以关闭借用检查", "RefCell moves checks to runtime, doesn't disable them"),
    ("RefCell 可以绕过 Rust 规则", "RefCell still enforces rules at runtime (panics on violation)"),
    ("Rust 不可能内存泄漏", "Rust can leak (Rc cycles, mem::forget, std::mem::ManuallyDrop)"),
    ("Rust 不会死锁", "Rust cannot prevent deadlocks at compile time"),
    ("Rust 可以自动避免死锁", "Rust cannot prevent deadlocks at compile time"),
    ("Rust 编译成功就不会有 Bug", "Type safety ≠ bug-free (logic errors still possible)"),
    ("异步就是多线程", "Async is concurrency, not necessarily parallelism"),
    ("Tokio 是 Rust 标准库", "Tokio is a third-party ecosystem library, not std"),
    ("unsafe 会关闭所有检查", "unsafe only enables 5 specific capabilities"),
    ("unsafe 就是不安全代码", "unsafe means the compiler can't verify safety; programmer must"),
    ("使用 unsafe 一定错误", "unsafe is sometimes necessary (FFI, optimization, safety abstractions)"),
    ("Cargo Feature 是运行时开关", "Cargo features are compile-time; not runtime toggles"),
    ("C++ 没有 RAII", "C++ invented RAII (constructors/destructors)"),
    ("只有 Rust 有智能指针", "C++ std::shared_ptr, std::unique_ptr are smart pointers"),
    ("Rust 一定比 C++ 快", "Performance depends on implementation; both are systems languages"),
    ("Python 一定比 Rust 慢", "Python with C extensions/numpy can be fast for specific workloads"),
]

inaccurate_hits = []
for dp, _, files in os.walk(ROOT):
    dp_rel = os.path.relpath(dp, ROOT)
    if "target" in dp_rel or ".git" in dp_rel or "audit_reports" in dp_rel:
        continue
    for f in files:
        if not f.endswith(".md"):
            continue
        path = os.path.join(dp, f)
        try:
            with open(path, "r", encoding="utf-8") as fh:
                for lineno, line in enumerate(fh, 1):
                    for pattern, explanation in INACCURATE_PATTERNS:
                        if pattern in line:
                            inaccurate_hits.append({
                                "file": os.path.relpath(path, ROOT),
                                "line": lineno,
                                "pattern": pattern,
                                "explanation": explanation,
                            })
        except Exception:
            pass

# ── 8.3 Background/motivation check for core chapters ─────────────────
CORE_CHAPTERS = [
    "04_stack_heap_and_raii", "05_ownership_move_copy_clone",
    "06_references_borrowing_slices", "09_enums_option_pattern_matching",
    "12_error_handling_result_question_mark", "15_generics_traits_trait_bounds",
    "16_lifetimes", "19_smart_pointers_box_rc_refcell",
    "21_threads_channels_shared_state", "22_async_await_tokio_intro",
    "24_unsafe_rust_and_ffi_overview",
]

MOTIVATION_KEYWORDS = [
    ("为什么需要", "why needed"),
    ("问题", "problem"),
    ("风险", "risk"),
    ("设计", "design"),
    ("原因", "reason"),
    ("动机", "motivation"),
    ("代价", "cost/tradeoff"),
    ("边界", "boundary/limitation"),
    ("局限", "limitation"),
    ("解决", "solve"),
    ("传统", "traditional"),
]

chapter_motivation = {}
for ch in CORE_CHAPTERS:
    readme = os.path.join(ROOT, "chapters", ch, "README.md")
    if not os.path.exists(readme):
        chapter_motivation[ch] = {"score": 0, "found": [], "missing": []}
        continue
    try:
        with open(readme, "r", encoding="utf-8") as fh:
            content = fh.read()
    except Exception:
        continue

    found_kws = []
    missing_kws = []
    for kw, desc in MOTIVATION_KEYWORDS:
        if kw in content:
            found_kws.append(desc)
        else:
            missing_kws.append(desc)
    score = len(found_kws)
    chapter_motivation[ch] = {"score": score, "found": found_kws, "missing": missing_kws}

# ── 8.4 Language comparison check ─────────────────────────────────────
COMPARISON_CHAPTERS = [
    "00_course_orientation", "01_hello_cargo", "02_variables_and_types",
    "03_functions_expressions_control_flow", "04_stack_heap_and_raii",
    "05_ownership_move_copy_clone", "06_references_borrowing_slices",
    "08_structs_methods_associated_functions", "09_enums_option_pattern_matching",
    "10_collections_vec_string_hashmap", "12_error_handling_result_question_mark",
    "13_packages_crates_modules_visibility", "15_generics_traits_trait_bounds",
    "16_lifetimes", "17_trait_objects_dynamic_dispatch", "18_closures_iterators",
    "19_smart_pointers_box_rc_refcell", "20_resource_management_drop_deref",
    "21_threads_channels_shared_state", "22_async_await_tokio_intro",
    "24_unsafe_rust_and_ffi_overview", "25_cargo_dependencies_features_profiles",
]

chapter_comparison = {}
for ch in COMPARISON_CHAPTERS:
    readme = os.path.join(ROOT, "chapters", ch, "README.md")
    if not os.path.exists(readme):
        chapter_comparison[ch] = "MISSING"
        continue
    try:
        with open(readme, "r", encoding="utf-8") as fh:
            content = fh.read()
    except Exception:
        chapter_comparison[ch] = "ERROR"
        continue

    has_python = "python" in content.lower()
    has_c = bool(re.search(r'\bC\b', content)) or "C语言" in content or "C 语言" in content
    has_cpp = "C++" in content or "Cpp" in content or "c++" in content.lower()

    comparison_count = sum([has_python, has_c, has_cpp])
    if comparison_count >= 2:
        chapter_comparison[ch] = "GOOD"
    elif comparison_count == 1:
        chapter_comparison[ch] = "PARTIAL"
    else:
        chapter_comparison[ch] = "MISSING"

# ── Report ─────────────────────────────────────────────────────────────
report_path = os.path.join(REPORT_DIR, "content_static_scan.md")
with open(report_path, "w", encoding="utf-8") as fh:
    fh.write("# Content Quality Static Scan\n\n")

    # Terminology
    fh.write("## 8.1 Core Terminology Coverage\n\n")
    fh.write(f"**Total terms checked**: {len(CORE_TERMS)}\n")
    fh.write(f"**Terms not found anywhere**: {len(term_missing)}\n")
    fh.write(f"**Terms without bilingual intro**: {len(missing_bilingual)}\n\n")
    if term_missing:
        fh.write("### Missing terms:\n")
        for t in term_missing:
            fh.write(f"- {t}\n")
        fh.write("\n")
    if missing_bilingual:
        fh.write("### Missing bilingual introduction:\n")
        for b in missing_bilingual:
            fh.write(f"- {b}\n")
        fh.write("\n")
    fh.write("### Term distribution:\n\n")
    fh.write("| Term | First Chapter | Other Chapters |\n")
    fh.write("|------|---------------|---------------|\n")
    for term in sorted(CORE_TERMS):
        chapters = term_chapters.get(term, [])
        first_ch = term_first_use.get(term, "NONE")
        others = [c for c in chapters if c != first_ch]
        fh.write(f"| {term} | {first_ch} | {', '.join(others[:5])} |\n")

    # Inaccurate statements
    fh.write(f"\n## 8.2 Inaccurate Statements: {len(inaccurate_hits)} hits\n\n")
    if inaccurate_hits:
        fh.write("| File | Line | Pattern | Issue |\n")
        fh.write("|------|------|---------|-------|\n")
        for hit in inaccurate_hits:
            fh.write(f"| `{hit['file']}` | {hit['line']} | `{hit['pattern']}` | {hit['explanation']} |\n")
    else:
        fh.write("No inaccurate statement patterns found.\n")

    # Background/motivation
    fh.write(f"\n## 8.3 Background & Motivation (Core Chapters)\n\n")
    fh.write("| Chapter | Score | Found | Missing |\n")
    fh.write("|---------|-------|-------|--------|\n")
    for ch in CORE_CHAPTERS:
        info = chapter_motivation.get(ch, {})
        fh.write(f"| {ch} | {info.get('score', 0)}/11 | {', '.join(info.get('found', [])[:5])} | {', '.join(info.get('missing', [])[:5])} |\n")

    # Language comparison
    fh.write(f"\n## 8.4 Language Comparison Coverage\n\n")
    fh.write("| Chapter | Status |\n")
    fh.write("|---------|--------|\n")
    for ch in COMPARISON_CHAPTERS:
        fh.write(f"| {ch} | {chapter_comparison.get(ch, 'UNKNOWN')} |\n")

    # Summary
    total_issues = len(inaccurate_hits) + len(term_missing) + sum(
        1 for v in chapter_motivation.values() if v.get("score", 0) < 5
    ) + sum(1 for v in chapter_comparison.values() if v == "MISSING")

    fh.write(f"\n## Summary\n\n")
    fh.write(f"- Core terms missing: {len(term_missing)}\n")
    fh.write(f"- Inaccurate statement hits: {len(inaccurate_hits)}\n")
    fh.write(f"- Chapters with weak motivation: {sum(1 for v in chapter_motivation.values() if v.get('score', 0) < 5)}\n")
    fh.write(f"- Chapters missing language comparisons: {sum(1 for v in chapter_comparison.values() if v == 'MISSING')}\n")

print(f"Content quality scan written to {report_path}")
print(f"Term missing={len(term_missing)}, Inaccurate={len(inaccurate_hits)}")

sys.exit(0)  # Non-zero only for explicit P0 issues; content scan is advisory

# Content Quality Static Scan

## 8.1 Core Terminology Coverage

**Total terms checked**: 45
**Terms not found anywhere**: 0
**Terms without bilingual intro**: 7

### Missing bilingual introduction:
- Move first in 15_generics_traits_trait_bounds but no Chinese '移动' nearby
- Copy first in 15_generics_traits_trait_bounds but no Chinese '复制' nearby
- Clone first in 15_generics_traits_trait_bounds but no Chinese '克隆' nearby
- Async first in 15_generics_traits_trait_bounds but no Chinese '异步' nearby
- Crate first in 15_generics_traits_trait_bounds but no Chinese '箱' nearby
- Slice first in 16_lifetimes but no Chinese '切片' nearby
- Workspace first in 25_cargo_dependencies_features_profiles but no Chinese 'Work空间' nearby

### Term distribution:

| Term | First Chapter | Other Chapters |
|------|---------------|---------------|
| Arc | 15_generics_traits_trait_bounds | 16_lifetimes, 16_lifetimes, 16_lifetimes, 10_collections_vec_string_hashmap, 10_collections_vec_string_hashmap |
| Async | 15_generics_traits_trait_bounds | 25_cargo_dependencies_features_profiles, 25_cargo_dependencies_features_profiles, 00_course_orientation, 00_course_orientation, 24_unsafe_rust_and_ffi_overview |
| Borrow Checker | 16_lifetimes | 06_references_borrowing_slices, 06_references_borrowing_slices, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 24_unsafe_rust_and_ffi_overview |
| Borrowing | 16_lifetimes | 06_references_borrowing_slices, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 05_ownership_move_copy_clone, 00_course_orientation |
| Clone | 15_generics_traits_trait_bounds | 16_lifetimes, 10_collections_vec_string_hashmap, 10_collections_vec_string_hashmap, 06_references_borrowing_slices, 06_references_borrowing_slices |
| Concurrency | 00_course_orientation | 02_variables_and_types, 22_async_await_tokio_intro, 21_threads_channels_shared_state, 21_threads_channels_shared_state |
| Copy | 15_generics_traits_trait_bounds | 16_lifetimes, 10_collections_vec_string_hashmap, 06_references_borrowing_slices, 06_references_borrowing_slices, 05_ownership_move_copy_clone |
| Crate | 15_generics_traits_trait_bounds | 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 16_lifetimes, 10_collections_vec_string_hashmap |
| Data Race | 06_references_borrowing_slices | 05_ownership_move_copy_clone, 00_course_orientation, 00_course_orientation, 00_course_orientation, 24_unsafe_rust_and_ffi_overview |
| Drop | 03_functions_expressions_control_flow | 12_error_handling_result_question_mark, 16_lifetimes, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 05_ownership_move_copy_clone |
| Dynamic Dispatch | 15_generics_traits_trait_bounds | 17_trait_objects_dynamic_dispatch, 17_trait_objects_dynamic_dispatch |
| Enum | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark |
| FFI | 03_functions_expressions_control_flow | 12_error_handling_result_question_mark, 25_cargo_dependencies_features_profiles, 00_course_orientation, 24_unsafe_rust_and_ffi_overview, 24_unsafe_rust_and_ffi_overview |
| Future | 17_trait_objects_dynamic_dispatch | 22_async_await_tokio_intro, 22_async_await_tokio_intro, 22_async_await_tokio_intro, 21_threads_channels_shared_state |
| Generic | 15_generics_traits_trait_bounds | 12_error_handling_result_question_mark, 16_lifetimes, 17_trait_objects_dynamic_dispatch, 17_trait_objects_dynamic_dispatch, 04_stack_heap_and_raii |
| Interior Mutability | 06_references_borrowing_slices | 19_smart_pointers_box_rc_refcell, 19_smart_pointers_box_rc_refcell |
| Lifetime | 15_generics_traits_trait_bounds | 16_lifetimes, 16_lifetimes, 16_lifetimes, 06_references_borrowing_slices, 05_ownership_move_copy_clone |
| Macro | 15_generics_traits_trait_bounds | 00_course_orientation, 24_unsafe_rust_and_ffi_overview, 01_hello_cargo, 26_workspace_architecture, 08_structs_methods_associated_functions |
| Module | 12_error_handling_result_question_mark | 13_packages_crates_modules_visibility, 13_packages_crates_modules_visibility, 13_packages_crates_modules_visibility, 17_trait_objects_dynamic_dispatch, 26_workspace_architecture |
| Monomorphization | 15_generics_traits_trait_bounds | 00_course_orientation, 00_course_orientation, 17_trait_objects_dynamic_dispatch, 18_closures_iterators |
| Move | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 16_lifetimes, 10_collections_vec_string_hashmap, 10_collections_vec_string_hashmap, 10_collections_vec_string_hashmap |
| Mutable Reference | 06_references_borrowing_slices | 04_stack_heap_and_raii |
| Mutex | 06_references_borrowing_slices | 05_ownership_move_copy_clone, 00_course_orientation, 24_unsafe_rust_and_ffi_overview, 24_unsafe_rust_and_ffi_overview, 19_smart_pointers_box_rc_refcell |
| Non-Lexical Lifetimes | 06_references_borrowing_slices |  |
| Option | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark |
| Ownership | 16_lifetimes | 06_references_borrowing_slices, 05_ownership_move_copy_clone, 05_ownership_move_copy_clone, 05_ownership_move_copy_clone, 00_course_orientation |
| Package | 12_error_handling_result_question_mark | 25_cargo_dependencies_features_profiles, 25_cargo_dependencies_features_profiles, 25_cargo_dependencies_features_profiles, 13_packages_crates_modules_visibility, 13_packages_crates_modules_visibility |
| Parallelism | 22_async_await_tokio_intro |  |
| Pattern Matching | 03_functions_expressions_control_flow | 11_patterns_and_destructuring, 09_enums_option_pattern_matching |
| RAII | 03_functions_expressions_control_flow | 12_error_handling_result_question_mark, 10_collections_vec_string_hashmap, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 00_course_orientation |
| Rc | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 16_lifetimes |
| RefCell | 06_references_borrowing_slices | 05_ownership_move_copy_clone, 24_unsafe_rust_and_ffi_overview, 19_smart_pointers_box_rc_refcell, 19_smart_pointers_box_rc_refcell, 19_smart_pointers_box_rc_refcell |
| Reference | 16_lifetimes | 06_references_borrowing_slices, 06_references_borrowing_slices, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 25_cargo_dependencies_features_profiles |
| Reference Counting | 19_smart_pointers_box_rc_refcell |  |
| Result | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 16_lifetimes |
| Runtime | 12_error_handling_result_question_mark | 17_trait_objects_dynamic_dispatch, 26_workspace_architecture, 19_smart_pointers_box_rc_refcell, 22_async_await_tokio_intro, 22_async_await_tokio_intro |
| Slice | 16_lifetimes | 10_collections_vec_string_hashmap, 06_references_borrowing_slices, 06_references_borrowing_slices, 05_ownership_move_copy_clone, 11_patterns_and_destructuring |
| Smart Pointer | 19_smart_pointers_box_rc_refcell | 20_resource_management_drop_deref |
| Static Dispatch | 15_generics_traits_trait_bounds | 17_trait_objects_dynamic_dispatch |
| Struct | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 03_functions_expressions_control_flow, 16_lifetimes, 16_lifetimes, 10_collections_vec_string_hashmap |
| Trait | 15_generics_traits_trait_bounds | 03_functions_expressions_control_flow, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 12_error_handling_result_question_mark, 16_lifetimes |
| Trait Bound | 15_generics_traits_trait_bounds | 16_lifetimes, 17_trait_objects_dynamic_dispatch, 04_stack_heap_and_raii, 08_structs_methods_associated_functions, 18_closures_iterators |
| Trait Object | 15_generics_traits_trait_bounds | 16_lifetimes, 17_trait_objects_dynamic_dispatch, 17_trait_objects_dynamic_dispatch, 19_smart_pointers_box_rc_refcell |
| Unsafe | 06_references_borrowing_slices | 24_unsafe_rust_and_ffi_overview, 24_unsafe_rust_and_ffi_overview, 24_unsafe_rust_and_ffi_overview, 01_hello_cargo, 19_smart_pointers_box_rc_refcell |
| Workspace | 25_cargo_dependencies_features_profiles | 13_packages_crates_modules_visibility, 13_packages_crates_modules_visibility, 01_hello_cargo, 26_workspace_architecture, 26_workspace_architecture |

## 8.2 Inaccurate Statements: 8 hits

| File | Line | Pattern | Issue |
|------|------|---------|-------|
| `MISCONCEPTIONS.md` | 52 | `Rust 不使用堆` | Rust uses heap via Box, Vec, String, etc. |
| `MISCONCEPTIONS.md` | 68 | `Move 就是深拷贝` | Move is ownership transfer, not deep copy |
| `MISCONCEPTIONS.md` | 380 | `Rust 可以自动避免死锁` | Rust cannot prevent deadlocks at compile time |
| `MISCONCEPTIONS.md` | 406 | `异步就是多线程` | Async is concurrency, not necessarily parallelism |
| `MENTAL_MODELS.md` | 287 | `Trait 就是接口` | Trait is not just an interface (has default impl, associated types, blanket impl) |
| `MENTAL_MODELS.md` | 326 | `Trait 就是接口` | Trait is not just an interface (has default impl, associated types, blanket impl) |
| `MENTAL_MODELS.md` | 464 | `异步就是多线程` | Async is concurrency, not necessarily parallelism |
| `MENTAL_MODELS.md` | 481 | `Option 就是 null` | Option is a sum type with compiler-enforced handling |

## 8.3 Background & Motivation (Core Chapters)

| Chapter | Score | Found | Missing |
|---------|-------|-------|--------|
| 04_stack_heap_and_raii | 5/11 | why needed, problem, design, reason, cost/tradeoff | risk, motivation, boundary/limitation, limitation, solve |
| 05_ownership_move_copy_clone | 5/11 | problem, design, cost/tradeoff, boundary/limitation, solve | why needed, risk, reason, motivation, limitation |
| 06_references_borrowing_slices | 5/11 | why needed, problem, design, boundary/limitation, solve | risk, reason, motivation, cost/tradeoff, limitation |
| 09_enums_option_pattern_matching | 7/11 | problem, design, reason, boundary/limitation, limitation | why needed, risk, motivation, cost/tradeoff |
| 12_error_handling_result_question_mark | 7/11 | why needed, problem, design, reason, cost/tradeoff | risk, motivation, limitation, traditional |
| 15_generics_traits_trait_bounds | 8/11 | why needed, problem, design, reason, cost/tradeoff | risk, motivation, limitation |
| 16_lifetimes | 8/11 | why needed, problem, risk, design, reason | motivation, limitation, traditional |
| 19_smart_pointers_box_rc_refcell | 6/11 | why needed, problem, risk, design, cost/tradeoff | reason, motivation, boundary/limitation, limitation, traditional |
| 21_threads_channels_shared_state | 9/11 | why needed, problem, risk, design, reason | motivation, limitation |
| 22_async_await_tokio_intro | 8/11 | why needed, problem, risk, design, reason | motivation, boundary/limitation, traditional |
| 24_unsafe_rust_and_ffi_overview | 6/11 | why needed, problem, risk, design, reason | motivation, cost/tradeoff, limitation, solve, traditional |

## 8.4 Language Comparison Coverage

| Chapter | Status |
|---------|--------|
| 00_course_orientation | GOOD |
| 01_hello_cargo | GOOD |
| 02_variables_and_types | GOOD |
| 03_functions_expressions_control_flow | GOOD |
| 04_stack_heap_and_raii | GOOD |
| 05_ownership_move_copy_clone | GOOD |
| 06_references_borrowing_slices | GOOD |
| 08_structs_methods_associated_functions | GOOD |
| 09_enums_option_pattern_matching | GOOD |
| 10_collections_vec_string_hashmap | GOOD |
| 12_error_handling_result_question_mark | GOOD |
| 13_packages_crates_modules_visibility | GOOD |
| 15_generics_traits_trait_bounds | GOOD |
| 16_lifetimes | GOOD |
| 17_trait_objects_dynamic_dispatch | GOOD |
| 18_closures_iterators | GOOD |
| 19_smart_pointers_box_rc_refcell | GOOD |
| 20_resource_management_drop_deref | GOOD |
| 21_threads_channels_shared_state | GOOD |
| 22_async_await_tokio_intro | GOOD |
| 24_unsafe_rust_and_ffi_overview | GOOD |
| 25_cargo_dependencies_features_profiles | GOOD |

## Summary

- Core terms missing: 0
- Inaccurate statement hits: 8
- Chapters with weak motivation: 0
- Chapters missing language comparisons: 0

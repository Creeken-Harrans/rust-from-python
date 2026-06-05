// ============================================================================
// Chapter 24: Unsafe Rust 与 FFI 概述
// ============================================================================

/// 演示裸指针 (Raw Pointer) 的创建和解引用
fn demo_raw_pointers() {
    println!("\n=== 1. 裸指针 (Raw Pointers) ===");

    let value = 42;
    let ptr_const: *const i32 = &value;
    let mut mut_value = 100;
    let ptr_mut: *mut i32 = &mut mut_value;

    println!("  ptr_const  地址: {:p}", ptr_const);
    println!("  ptr_mut    地址: {:p}", ptr_mut);

    // SAFETY: ptr_const 指向有效的 i32 值
    unsafe {
        println!("  *ptr_const = {}", *ptr_const);
    }

    // SAFETY: ptr_mut 指向有效的可变 i32
    unsafe {
        *ptr_mut = 200;
        println!("  修改后 *ptr_mut = {}", *ptr_mut);
    }
}

/// 演示: 调用 unsafe 函数 (Rust 2024 中 unsafe fn body 也需要 unsafe 块)
unsafe fn dangerous_operation(ptr: *const i32) -> i32 {
    // SAFETY: 调用者保证 ptr 有效
    unsafe { *ptr }
}

fn demo_unsafe_function() {
    println!("\n=== 2. 调用 Unsafe 函数 ===");

    let value = 42;
    let ptr: *const i32 = &value;

    // SAFETY: ptr 指向有效的 i32 值
    let result = unsafe { dangerous_operation(ptr) };
    println!("  dangerous_operation 返回: {}", result);
}

/// 演示安全抽象 (Safe Abstraction)
fn demo_safe_abstraction() {
    println!("\n=== 3. 安全抽象 (Safe Abstraction) ===");

    let mut data = vec![1, 2, 3, 4, 5, 6];

    fn split_at_mut_safe<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        assert!(mid <= slice.len());
        let len = slice.len();
        let ptr = slice.as_mut_ptr();
        // SAFETY: ptr 有效, mid <= len, 两个子切片不重叠
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let (left, right) = split_at_mut_safe(&mut data, 3);
    println!("  left  = {:?}", left);
    println!("  right = {:?}", right);

    left[0] = 100;
    right[0] = 200;
    println!("  修改后 data = {:?}", data);
}

/// 五类 Unsafe 能力说明
fn five_unsafe_abilities() {
    println!("\n=== 4. Unsafe 的五种能力 ===");

    println!("  1. 解引用裸指针 (*const T 和 *mut T)");
    println!("  2. 调用 unsafe 函数或方法");
    println!("  3. 访问或修改可变静态变量 (static mut)");
    println!("  4. 实现 unsafe trait (如 Send, Sync)");
    println!("  5. 访问联合体 (Union) 的字段");
}

/// FFI 概述
fn ffi_overview() {
    println!("\n=== 5. FFI 概述 ===");

    println!("  FFI 允许 Rust 与其他语言 (主要是 C) 互操作:");
    println!();
    println!("  Rust 调用 C:");
    println!("    extern {{ fn some_c_function(x: i32) -> i32; }}");
    println!("    let result = unsafe {{ some_c_function(42) }};");
    println!();
    println!("  C 调用 Rust:");
    println!("    #[no_mangle]");
    println!("    pub extern fn rust_function() {{ ... }}");
    println!();
    println!("  FFI 本质上是 unsafe 的, 因为跨越了 Rust 的安全边界");
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║          Unsafe Rust 与 FFI 概述                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");

    println!();
    println!("Unsafe Rust 意味着:");
    println!("  - 我已经手动验证了这段代码的内存安全性");
    println!("  - 编译器请信任我，我知道自己在做什么");
    println!("  - 我负责维护 Rust 的安全不变量");
    println!();
    println!("Unsafe Rust 不会关闭借用检查器或其他安全检查。");
    println!("Rust 的核心理念:");
    println!("  用最小的 unsafe 内核构建安全的抽象。");

    demo_raw_pointers();
    demo_unsafe_function();
    demo_safe_abstraction();
    five_unsafe_abilities();
    ffi_overview();

    println!();
    println!("总结:");
    println!("  1. Unsafe Rust 是安全 Rust 的扩展，而非替代品");
    println!("  2. 用 unsafe 构建安全 API 包装是 Rust 的惯用模式");
    println!("  3. 每个 unsafe 块都需要 SAFETY 注释");
    println!("  4. FFI 本质上是 unsafe 的");
    println!("  5. 初学者不应使用 unsafe 来绕过借用检查器");
}

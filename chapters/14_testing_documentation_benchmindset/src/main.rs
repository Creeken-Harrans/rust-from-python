fn main() {
    println!("=== 测试与文档演示库 ===");
    println!("运行 cargo test 来执行所有测试");
    println!("运行 cargo doc --open 来查看文档");

    // Demonstrate a few functions
    println!("\nadd(2, 3) = {}", testing_and_docs::add(2, 3));
    println!(
        "word_count(\"Hello Rust\") = {}",
        testing_and_docs::word_count("Hello Rust")
    );
    println!(
        "is_palindrome(\"radar\") = {}",
        testing_and_docs::is_palindrome("radar")
    );
}

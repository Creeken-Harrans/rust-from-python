#[test]
fn integration_test_add_and_multiply() {
    let sum = testing_and_docs::add(5, 7);
    let product = testing_and_docs::multiply(sum, 2);
    assert_eq!(product, 24);
}

#[test]
fn integration_test_word_operations() {
    let text = "The quick brown fox";
    assert_eq!(testing_and_docs::word_count(text), 4);
    assert!(!testing_and_docs::is_palindrome(text));
}

#[test]
fn integration_test_palindrome_workflow() {
    let words = ["radar", "level", "hello", "world", "civic"];
    let palindrome_count: usize = words
        .iter()
        .filter(|w| testing_and_docs::is_palindrome(w))
        .count();
    assert_eq!(palindrome_count, 3);
}

fn main() {
    let diff = diff::Diff::new("test_lines.txt", "test_lines_modified.txt");
    diff.print_diff();
}

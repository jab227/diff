fn main() {
    let original = diff::files::read_file_lines("test_lines.txt");
    let modified = diff::files::read_file_lines("test_lines_modified.txt");

    let diff = diff::Diff::new(&original, &modified);
    diff.print_diff();
}

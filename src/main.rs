use diff;
fn main() {
    let original = diff::files::read_file_lines("test_lines.txt");
    let modified = diff::files::read_file_lines("test_lines_modified.txt");

    let grid = diff::lcs::lcs(&original,&modified);
    diff::lcs::print_diff(grid, &original, &modified, original.len(), modified.len());
}



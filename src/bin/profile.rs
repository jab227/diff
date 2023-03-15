use diffy::diff;

fn main() {
    let original = include_str!("../../original.txt")
        .split('\n')
        .collect::<Vec<&str>>();
    let modified = include_str!("../../modified.txt")
        .split('\n')
        .collect::<Vec<&str>>();
    let diff = diff::Diff::new(&original, &modified);
    let _result = diff.generate_diff();
    //    println!("{}",result);
}

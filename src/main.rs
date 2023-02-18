mod commit;

use commit::Commit;

fn main() {
    let commit = Commit::builder()
        .reverse()
        .optionnal()
        .types(vec!["feat", "fix"])
        .scopes()
        .optionnal()
        .breaking()
        .optionnal()
        .colon()
        .description(1, 52)
        .build();
    println!("{}", commit.regex);
}

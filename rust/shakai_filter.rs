fn main()
{
    const INVALID_WORDS: &[&str] = &["殺す", "人はなぜ", "死ね", "疲れた", "つらい"];

    let words = vec!["もうつらい", "殺す", "人はなぜ生きるのか", "元気です", "疲れたので帰りたい", "楽しい"];

    let fil = words
        .iter()
        .map(|msg|
             match INVALID_WORDS.iter().any(|word| msg.contains(word)) {
                 true  => "にゃーん",
                 false => msg,
             }
            );

    for i in fil {
        println!("{}", i)
    }
}

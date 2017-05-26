// http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

struct Object(usize);

impl Object {
    fn new(n: usize) -> Object
    {
        Object(n)
    }

    fn get(&self) -> usize
    {
        self.0
    }
}

fn main()
{
    sample_iter();
    sample_iter_mut();
    sample_into_iter();
}


fn sample_iter()
{
    let objects = vec![
        Object::new(1),
        Object::new(2),
        Object::new(3),
    ];

    let sum: usize = objects
        .iter()
        .map(|o: &Object| o.get())
        .fold(0, |acc, len| acc + len );
    assert_eq!(sum, 6);

    // 借用なので再び使用可能
    // into_iterではmoveしてしまうのでできない
    let sum: Vec<usize> = objects
        .iter()
        .map(|o: &Object| o.get())
        .collect();
    assert_eq!(sum, vec![1, 2, 3]);

    let player_scores = [
        ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19),
    ];

    let players = player_scores
        .iter()
        // この場合はクロージャの型をつけなくてもいいが、
        // 型をバラしたいときは&をつけて参照であることを明示する必要がある
        .map(|&(player, _)| {
            player
        })
        .collect::<Vec<_>>();

    assert_eq!(players, ["Jack", "Jane", "Jill", "John"]);
    println!("{:?}", player_scores)
}


fn sample_iter_mut()
{
    let mut teams = [
        [ ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19), ],
        [ ("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17), ]
    ];

    let teams_in_score_order = teams
        .iter_mut()
        .map(|team| {
            // sort_byはmutableな操作
            team.sort_by(|&a, &b| a.1.cmp(&b.1).reverse());
            team
        })
    .collect::<Vec<_>>();

    println!("Teams: {:?}", teams_in_score_order);
}


fn sample_into_iter()
{
    fn get_names(v: Vec<(String, usize)>) -> Vec<String> {
        v.into_iter()
            .map(|(name, _)| name)
            .collect()
    }

    let v = vec!( ("Herman".to_string(), 5));
    let names = get_names(v);

    assert_eq!(names, ["Herman"]);
}

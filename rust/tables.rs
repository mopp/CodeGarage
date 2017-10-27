trait TableLevel {
}

trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

struct Level4;
struct Level3;
struct Level2;
struct Level1;

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

impl HierarchicalLevel for Level4 { type NextLevel = Level3; }
impl HierarchicalLevel for Level3 { type NextLevel = Level2; }
impl HierarchicalLevel for Level2 { type NextLevel = Level1; }

use std::marker::PhantomData;

struct Table<L: TableLevel> {
    entries: [usize; 1024],
    level: PhantomData<L>,
}

impl<L> Table<L> where L: HierarchicalLevel
{
    fn new() -> Table<L>
    {
        Table {
            entries: [0; 1024],
            level: PhantomData,
        }
    }

    fn next_table(&self, _: usize) -> Option<&Table<L::NextLevel>>
    {
        None
    }

    fn dummy(&self) -> Option<usize>
    {
        Some(10)
    }
}

impl Table<Level1> {
}


fn main()
{
    let table_l4: Table<Level4> = Table::new();
    // let tmp = table_l4
    //     .next_table(0)
    //     .and_then(|p3: &Table<Level3>| p3.next_table(0))
    //     .and_then(|p2: &Table<Level2>| p2.dummy())
    //     .or_else(|| Some(10));
}

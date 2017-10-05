type TopModuleType = usize;


mod submodule_a {
    pub fn foo() {
        println!("here is SubmoduleA::foo");
    }

    pub mod subsubmodule_a {
        pub fn foo() {
            use TopModuleType;

            let n: TopModuleType = 200;
            println!("here is SubmoduleA::foo: {}", n);
        }
    }
}


fn main() {
    let n: TopModuleType = 100;
    println!("{}", n);

    submodule_a::foo();
    submodule_a::subsubmodule_a::foo();
}

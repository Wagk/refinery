#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn matchmaking()
    {
        enum num {
            SomeTuple(i32, i32, i32),
            SomeStruct{a: i32, b:i32},
            SomeFlag
        }

        // let sum = num::SomeTuple(1, 2, 3);
        let sum = num::SomeFlag;
        // let sum = num::SomeStruct{a:1, b:2};

        // The values are aliases that we can use. It appears to be a
        // destructuring call of some sort
        match sum {
            num::SomeTuple(x, y, z) => println!("some tuple ({}, {}, {})", x, y, z),
            num::SomeStruct{a: x, b: y} => println!("some struct {}, {}", x, y),
            num::SomeFlag => println!("some flag"),
            _ => println!("Not some tuple")
        }
    }

}

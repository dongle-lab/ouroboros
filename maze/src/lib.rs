use rand::prelude::*;

pub fn gen_maze() {
    let mut rng = rand::rng();

    println!("char: '{}'", rng.random::<char>());
    println!("alpha: '{}'", rng.sample(rand::distr::Alphanumeric) as char);

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    let slect = nums.choose(&mut rng).unwrap();
    println!("nums 1~100: '{}'", slect);
}

#[cfg(test)]
mod maze_test {
    use super::*;

    #[test]
    fn foo() {
        gen_maze();
    }
}

fn main() {
    let top_10 = get_top_10_recipes_after_nth_recipe(236021);

    for n in top_10 {
        print!("{}", n);
    }

    println!("");
    println!("{}", how_many_on_the_left_of_seq(vec![2,3,6,0,2,1]));
}

fn how_many_on_the_left_of_seq(seq: Vec<usize>) -> usize {
    let (mut recipes, mut elf_a, mut elf_b) = (vec![3, 7], 0, 1);

    loop {
        let score_a = recipes[elf_a];
        let score_b = recipes[elf_b];

        let nadded = add_recipe(score_a, score_b, &mut recipes);

        let len = recipes.len();
        if len > seq.len() {
            if recipes.get(len - seq.len()..len).unwrap().iter()
                .zip(seq.iter())
                .all(|(a, b)| a == b) { break len - seq.len() }

            if nadded == 2 && recipes.get(len - seq.len() - 1..len - 1).unwrap().iter()
                .zip(seq.iter())
                .all(|(a, b)| a == b) { break len - seq.len() - 1 }
        }

        elf_a = (elf_a + score_a + 1) % recipes.len();
        elf_b = (elf_b + score_b + 1) % recipes.len();
    }
}

fn get_top_10_recipes_after_nth_recipe(n: usize) -> Vec<usize> {
    let (mut recipes, mut elf_a, mut elf_b) = (vec![3, 7], 0, 1);

    let mut nrecipes = 2;
    loop {
        let score_a = recipes[elf_a];
        let score_b = recipes[elf_b];

        nrecipes += add_recipe(score_a, score_b, &mut recipes);

        if nrecipes >= n + 11 { break recipes.drain(n..n+10).collect() }

        elf_a = (elf_a + score_a + 1) % recipes.len();
        elf_b = (elf_b + score_b + 1) % recipes.len();

    }
}

fn add_recipe(score_a: usize, score_b: usize, recipes: &mut Vec<usize>) -> usize {
    let new_recipe = score_a + score_b;
    match new_recipe > 9 {
        true => {
            recipes.push(1);
            recipes.push(new_recipe % 10);
            2
        },
        false => {
            recipes.push(new_recipe);
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_10() {
        assert_eq!(get_top_10_recipes_after_nth_recipe(9), vec![5,1,5,8,9,1,6,7,7,9]);
        assert_eq!(get_top_10_recipes_after_nth_recipe(5), vec![0,1,2,4,5,1,5,8,9,1]);
        assert_eq!(get_top_10_recipes_after_nth_recipe(18), vec![9,2,5,1,0,7,1,0,8,5]);
        assert_eq!(get_top_10_recipes_after_nth_recipe(2018), vec![5,9,4,1,4,2,9,8,8,2]);
    }

    #[test]
    fn test_how_many_left() {
        assert_eq!(how_many_on_the_left_of_seq(vec![5,1,5,8,9]), 9);
        assert_eq!(how_many_on_the_left_of_seq(vec![0,1,2,4,5]), 5);
        assert_eq!(how_many_on_the_left_of_seq(vec![9,2,5,1,0]), 18);
        assert_eq!(how_many_on_the_left_of_seq(vec![5,9,4,1,4]), 2018);
    }
}

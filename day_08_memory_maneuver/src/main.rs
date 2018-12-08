use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;

fn main() {
    let mut f = File::open("data/day_08_input").unwrap();

    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let buffer = buffer.replace("\n", "");

    let tree = parse_string_to_vec(&buffer);

    let (metadata, _) = gather_metadata(tree);
    let sum: usize = metadata.iter().sum();

    println!("Sum of metadata values: {}", sum);
}

fn gather_metadata(mut tree: VecDeque<usize>) -> (Vec<usize>, VecDeque<usize>) {
    let nchild = tree.pop_front().unwrap();
    let nmeta = tree.pop_front().unwrap();

    match nchild {
        0 => {
            take_n_metavalues(tree, nmeta)
        },
        _ => {
            let (mut metadata, new_tree) = (0..nchild)
                .fold((Vec::new(), tree), |(mut metadata, new_tree), _| {
                    let (mut m, t) = gather_metadata(new_tree);
                    metadata.append(&mut m);

                    (metadata, t)
                });

            let (mut metavalues, new_tree) = take_n_metavalues(new_tree, nmeta);
            metadata.append(&mut metavalues);

            (metadata, new_tree)
        }
    }

}

fn take_n_metavalues(mut tree: VecDeque<usize>, n: usize) -> (Vec<usize>, VecDeque<usize>) {
    let values = (0..n)
        .fold(Vec::new(), |mut acc, _| {
            let x = tree.pop_front().unwrap();
            acc.push(x);
            acc
        });

    (values, tree)
}

fn parse_string_to_vec(x: &str) -> VecDeque<usize> {
    x.split(" ")
        .map(|x| -> usize { x.parse().unwrap() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gather_metadata() {
        let tree = parse_string_to_vec("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let (metadata, _) = gather_metadata(tree);
        let sum: usize = metadata.iter().sum();

        assert_eq!(sum, 138);
    }
}

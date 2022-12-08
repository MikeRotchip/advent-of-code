use std::fs;

fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut trees = vec![vec![0; 99]; 99];

    let mut line = 0;
    let mut column = 0;

    for tree in input.chars() {
        if tree == '\n' {
            line += 1;
            column = 0;

            continue;
        }

        let (i, j) = (column, line);

        trees[i][j] = tree.to_string().parse::<i32>().unwrap();

        column += 1;
    }

    let VECTORS: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let EDGES = vec![0, 98];

    let mut max_scenic_score = 0;

    for i in 0..99 {
        for j in 0..99 {
            if EDGES
                .iter()
                .any(|edge| *edge as usize == i || *edge as usize == j)
            {
                continue;
            }

            let my_height = trees[i][j];

            let mut my_scenic_score = 1;

            for vector in VECTORS.iter() {
                for size in 0..99 {
                    let (x, y) = vector;

                    let (m, n) = (
                        (i as i32 + (x * size)) as usize,
                        (j as i32 + (y * size)) as usize,
                    );

                    println!("tree - [{},{}], looking at - [{},{}]", i, j, m, n);

                    if ((m != i || n != j) && trees[m][n] >= my_height)
                        || EDGES
                            .iter()
                            .any(|edge| *edge as usize == m || *edge as usize == n)
                    {
                        my_scenic_score *= i.abs_diff(m) + j.abs_diff(n);

                        break;
                    }
                }
            }

            if my_scenic_score > max_scenic_score {
                max_scenic_score = my_scenic_score;
            }
        }
    }

    println!("{}", max_scenic_score);
}

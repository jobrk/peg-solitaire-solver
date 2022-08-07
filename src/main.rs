use std::collections::HashSet;

type Board = [[i8; 7]; 7];
type Move = (char, usize, usize);

fn print_board(board: &Board) {
    for line in board {
        let mut cur_line: String = "".to_owned();
        for elt in line {
            match elt {
                -1 => cur_line.push_str(" "),
                0 => cur_line.push_str("o"),
                1 => cur_line.push_str("."),
                _ => {}
            }
        }
        println!("{}", cur_line);
    }
}

fn print_solution(initial_board: Board, solution: &Vec<Move>) {
    let mut board = initial_board;
    println!("Initial board");
    print_board(&board);
    for (i, mov) in solution.iter().enumerate() {
        board = play_move(board, *mov);
        println!("\nMove {}", i);
        print_board(&board)
    }
}

fn count_balls(board: &Board) -> u32 {
    let mut count = 0;
    for line in board {
        for elt in line {
            if *elt == 1 {
                count += 1;
            }
        }
    }

    count
}

fn play_move(mut board: Board, mov: Move) -> Board {
    let d = mov.0;
    let i = mov.1;
    let j = mov.2;
    for k in 0..3 {
        if d == 'h' {
            board[i][j + k] = 1 - board[i][j + k];
        }
        if d == 'v' {
            board[i + k][j] = 1 - board[i + k][j]
        }
    }

    board
}

fn get_available_moves(board: &Board) -> Vec<Move> {
    let mut moves = vec![];
    let m = board.len();
    let n = board[0].len();

    for i in 0..m {
        for j in 0..(n - 2) {
            if board[i][j + 1] == 1 {
                if board[i][j] == 0 && board[i][j + 2] == 1 {
                    moves.push(('h', i, j));
                } else if board[i][j] == 1 && board[i][j + 2] == 0 {
                    moves.push(('h', i, j));
                }
            }
        }
    }

    for i in 0..(m - 2) {
        for j in 0..n {
            if board[i + 1][j] == 1 {
                if board[i][j] == 0 && board[i + 2][j] == 1 {
                    moves.push(('v', i, j));
                } else if board[i][j] == 1 && board[i + 2][j] == 0 {
                    moves.push(('v', i, j));
                }
            }
        }
    }

    moves
}

fn get_solutions(board: Board, single: bool) -> Vec<Vec<Move>> {
    let mut solutions = vec![];
    let ball_count = count_balls(&board);
    let mut cache: HashSet<Board> = HashSet::new();

    get_solutions_helper(
        board,
        vec![],
        &mut solutions,
        &single,
        &ball_count,
        &mut cache,
    );

    solutions
}

fn get_solutions_helper(
    board: Board,
    path: Vec<Move>,
    solutions: &mut Vec<Vec<Move>>,
    single: &bool,
    ball_count: &u32,
    cache: &mut HashSet<Board>,
) {
    if *single && solutions.len() >= 1 {
        return;
    }
    if *single && cache.contains(&board) {
        return;
    }
    cache.insert(board);
    if path.len() as u32 == *ball_count - 1 {
        solutions.push(path);
        return;
    }
    let moves = get_available_moves(&board);
    for mov in moves {
        let new_board = play_move(board, mov);
        let mut new_path: Vec<Move> = path.clone();
        new_path.push(mov);
        get_solutions_helper(new_board, new_path, solutions, single, ball_count, cache);
    }
}

fn main() {
    let board: Board = [
        [-1, -1, 1, 1, 1, -1, -1],
        [-1, -1, 1, 1, 1, -1, -1],
        [1, 1, 1, 1, 1, 1, 1],
        [1, 1, 1, 0, 1, 1, 1],
        [1, 1, 1, 1, 1, 1, 1],
        [-1, -1, 1, 1, 1, -1, -1],
        [-1, -1, 1, 1, 1, -1, -1],
    ];
    let solutions = get_solutions(board, true);
    print_solution(board, solutions.get(0).unwrap())
}

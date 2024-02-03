extern crate rand;
use rand::Rng;
use std::io;

#[derive(Clone, Copy)]
enum Face {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}

struct Cube {
    up: Vec<Vec<i32>>,
    down: Vec<Vec<i32>>,
    front: Vec<Vec<i32>>,
    back: Vec<Vec<i32>>,
    left: Vec<Vec<i32>>,
    right: Vec<Vec<i32>>,
}

impl Cube {
    fn new() -> Self {
        // Initialize each face with unique numbers for each square
        Cube {
            up: vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]],
            left: vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]],
            front: vec![vec![3, 3, 3], vec![3, 3, 3], vec![3, 3, 3]],
            right: vec![vec![4, 4, 4], vec![4, 4, 4], vec![4, 4, 4]],
            back: vec![vec![5, 5, 5], vec![5, 5, 5], vec![5, 5, 5]],
            down: vec![vec![6, 6, 6], vec![6, 6, 6], vec![6, 6, 6]],
        }
    }

    fn rotate_face_clockwise(&mut self, face: Face) {
        match face {
            Face::Front => {
                println!("Rotating Front face");
                self.rotate_front_face();
                self.rotate_adjacent_edges(Face::Front);
            }
            Face::Up => {
                println!("Rotating Up face");
                self.rotate_up_face();
                self.rotate_adjacent_edges(Face::Up);
            }
            Face::Down => {
                println!("Rotating Down face");
                self.rotate_down_face();
                self.rotate_adjacent_edges(Face::Down);
            }
            Face::Back => {
                println!("Rotating Back face");
                self.rotate_back_face();
                self.rotate_adjacent_edges(Face::Back);
            }
            Face::Left => {
                println!("Rotating Left face");
                self.rotate_left_face();
                self.rotate_adjacent_edges(Face::Left);
            }
            Face::Right => {
                println!("Rotating Right face");
                self.rotate_right_face();
                self.rotate_adjacent_edges(Face::Right);
            }
            _ => { /* Implement rotations for other faces */ }
        }
    }

    fn rotate_face(&mut self, face: Face, direction: char) {
        match direction {
            'r' => self.rotate_face_clockwise(face),
            'l' => {
                for _ in 0..3 {
                    self.rotate_face_clockwise(face);
                }
            }
            _ => println!("Invalid direction"),
        }
    }

    fn rotate_front_face(&mut self) {
        let temp = self.front.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.front[j][2 - i] = temp[i][j];
            }
        }
    }

    fn rotate_up_face(&mut self) {
        let temp = self.up.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.up[j][2 - i] = temp[i][j]
            }
        }
    }

    fn rotate_down_face(&mut self) {
        let temp = self.down.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.down[j][2 - i] = temp[i][j]
            }
        }
    }

    fn rotate_back_face(&mut self) {
        let temp = self.back.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.back[j][2 - i] = temp[i][j]
            }
        }
    }

    fn rotate_left_face(&mut self) {
        let temp = self.left.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.left[j][2 - i] = temp[i][j]
            }
        }
    }

    fn rotate_right_face(&mut self) {
        let temp = self.right.clone();
        for i in 0..3 {
            for j in 0..3 {
                self.right[j][2 - i] = temp[i][j]
            }
        }
    }

    fn rotate_adjacent_edges(&mut self, face: Face) {
        match face {
            Face::Front => {
                let temp_up = self.up[2].clone();
                let temp_down = self.down[0].clone();
                let temp_left = self.left.iter().map(|row| row[2]).collect::<Vec<_>>();
                let temp_right = self.right.iter().map(|row| row[0]).collect::<Vec<_>>();
                for i in 0..3 {
                    self.up[2][i] = temp_left[2 - i];
                    self.down[0][i] = temp_right[i];

                    self.left[i][2] = temp_down[i];
                    self.right[i][0] = temp_up[2 - i];
                }
            }

            Face::Back => {
                let temp_up = self.up[0].clone();
                let temp_down = self.down[2].clone();
                let temp_left = self.left.iter().map(|row| row[0]).collect::<Vec<_>>();
                let temp_right = self.right.iter().map(|row| row[2]).collect::<Vec<_>>();

                for i in 0..3 {
                    self.up[2][i] = temp_right[2 - i];
                    self.down[0][i] = temp_left[i];
                    self.left[i][2] = temp_up[i];
                    self.right[i][0] = temp_down[2 - i];
                }
            }

            Face::Left => {
                let temp_up = self.up.iter().map(|row| row[0]).collect::<Vec<_>>();
                let temp_down = self.down.iter().map(|row| row[0]).collect::<Vec<_>>();
                let temp_back = self.back.iter().map(|row| row[2]).collect::<Vec<_>>();
                let temp_front = self.front.iter().map(|row| row[0]).collect::<Vec<_>>();

                for i in 0..3 {
                    self.up[i][0] = temp_back[2 - i];
                    self.down[i][0] = temp_front[i];
                    self.back[i][2] = temp_down[2 - i];
                    self.front[i][0] = temp_up[i];
                }
            }

            Face::Right => {
                let temp_up = self.up.iter().map(|row| row[2]).collect::<Vec<_>>();
                let temp_down = self.down.iter().map(|row| row[2]).collect::<Vec<_>>();
                let temp_back = self.back.iter().map(|row| row[0]).collect::<Vec<_>>();
                let temp_front = self.front.iter().map(|row| row[2]).collect::<Vec<_>>();

                for i in 0..3 {
                    self.up[i][2] = temp_front[i];
                    self.down[i][2] = temp_back[2 - i];
                    self.front[i][2] = temp_up[i];
                    self.back[i][0] = temp_down[2 - i];
                }
            }
            Face::Up => {
                let temp_front = self.front[0].clone();
                let temp_back = self.back[0].clone();
                let temp_left = self.left[0].clone();
                let temp_right = self.right[0].clone();

                for i in 0..3 {
                    self.front[0][i] = temp_right[i];
                    self.right[0][i] = temp_back[i];
                    self.back[0][i] = temp_left[i];
                    self.left[0][i] = temp_front[i];
                }
            }
            Face::Down => {
                let temp_front = self.front[2].clone();
                let temp_back = self.back[2].clone();
                let temp_left = self.left[2].clone();
                let temp_right = self.right[2].clone();

                for i in 0..3 {
                    self.front[2][i] = temp_right[i];
                    self.right[2][i] = temp_back[i];
                    self.back[2][i] = temp_left[i];
                    self.left[2][i] = temp_front[i];
                }
            }
        }
    }

    fn print_colored_number(&self, number: i32) {
        let (color_code, color_reset) = match number {
            1 => ("\x1b[47m", "\x1b[0m"),  // White for Up
            2 => ("\x1b[43m", "\x1b[0m"),  // Orange for Left (Using yellow to simulate orange)
            3 => ("\x1b[42m", "\x1b[0m"),  // Green for Front
            4 => ("\x1b[41m", "\x1b[0m"),  // Red for Right
            5 => ("\x1b[44m", "\x1b[0m"),  // Blue for Back
            6 => ("\x1b[103m", "\x1b[0m"), // Yellow (Bright) for Down
            _ => ("\x1b[0m", "\x1b[0m"),   // Default (No color)
        };
        print!("{}  {}{}", color_code, number, color_reset);
    }

    fn print_cube_with_colors(&self) {
        // Print the 'up' face with color
        println!("          Up:");
        for row in &self.up {
            print!("          "); // Align with the front face
            for &number in row {
                self.print_colored_number(number);
            }
            println!();
        }
        println!(); // Spacer

        // Print 'left', 'front', 'right', and 'back' faces in a row with color
        println!("Left:       Front:      Right:      Back:");
        for i in 0..3 {
            for &number in &self.left[i] {
                self.print_colored_number(number);
            }
            print!("  "); // Spacer between faces
            for &number in &self.front[i] {
                self.print_colored_number(number);
            }
            print!("  "); // Spacer between faces
            for &number in &self.right[i] {
                self.print_colored_number(number);
            }
            print!("  "); // Spacer between faces
                          // Note: The back face is printed in reversed order for correct orientation
            for j in (0..3).rev() {
                self.print_colored_number(self.back[i][j]);
            }
            println!();
        }
        println!(); // Spacer

        // Print the 'down' face with color
        println!("          Down:");
        for row in &self.down {
            print!("          "); // Align with the front face
            for &number in row {
                self.print_colored_number(number);
            }
            println!();
        }
        println!(); // Spacer
    }
}

fn generate_random_array() -> [i32; 20] {
    let mut rng = rand::thread_rng();
    let mut array = [0; 20];

    for i in 0..array.len() {
        array[i] = rng.gen_range(0..=5);
    }
    array
}

fn match_face(input: &str) -> Option<Face> {
    match input.to_lowercase().as_str() {
        "up" => Some(Face::Up),
        "left" => Some(Face::Left),
        "front" => Some(Face::Front),
        "right" => Some(Face::Right),
        "back" => Some(Face::Back),
        "down" => Some(Face::Down),
        _ => None,
    }
}

fn main() {
    let mut cube = Cube::new();
    let shuffle_array = generate_random_array();

    for &action in shuffle_array.iter() {
        let face = match action {
            0 => Face::Up,
            1 => Face::Down,
            2 => Face::Front,
            3 => Face::Back,
            4 => Face::Left,
            5 => Face::Right,
            _ => continue,
        };
        cube.rotate_face_clockwise(face);
    }

    loop {
        println!("Enter face to rotate (up, down, left, front, right, back) or 'q' to quit:");
        let mut face_input = String::new();
        io::stdin()
            .read_line(&mut face_input)
            .expect("Failed to read line");
        let face_input = face_input.trim();

        if face_input.eq_ignore_ascii_case("q") {
            break;
        }

        if let Some(face) = match_face(face_input) {
            println!("Enter direction of rotation ('l' = counter-clockwise, 'r' = clockwise)");
            let mut direction_input = String::new();
            io::stdin()
                .read_line(&mut direction_input)
                .expect("Failed to read line");
            let direction_input = direction_input.trim().chars().next().unwrap_or(' ');

            cube.rotate_face(face, direction_input);
            cube.print_cube_with_colors();
        } else {
            println!("Invalid face input.")
        }
    }
}

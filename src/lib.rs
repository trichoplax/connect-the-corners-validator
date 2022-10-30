use itertools::Itertools;
use wasm_bindgen::prelude::*;

#[derive(PartialEq, Clone, Copy)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    pub fn new(x: isize, y: isize) -> Self {
        Location {x, y}
    }
    
    fn plus(&self, o: Offset) -> Self {
        Location {x: self.x + o.x, y: self.y + o.y}
    }
}

struct Offset {
    x: isize,
    y: isize,
}

impl Offset {
    pub fn new(x: isize, y: isize) -> Self {
        Offset {x, y}
    }
    
    fn times(&self, multiplier: isize) -> Self {
        Offset {x: self.x * multiplier, y: self.y * multiplier}
    }
}

#[wasm_bindgen]
pub fn validate_connect_the_corners(s: &str) -> String {
    if s.len() == 0 {
        return "Invalid. The input is empty.".to_string();
    }

    if distinct_characters(s).len() > 2 {
        return "Invalid. The input contains more than 2 distinct characters.".to_string();
    }

    if !rectangle(s) {
        return "Invalid. The input is not rectangular.".to_string();
    }
    
    let path_character = character_connecting_opposite_corners(s);
    
    if path_character.is_none() {
        return "Invalid. There are not 2 diagonally opposite corners connected by horizontally and vertically adjacent characters.".to_string();
    }
    
    match lines_of_4(path_character.unwrap(), s) {
        Some(highlighted_lines) => return format!("Invalid. There must not be 4 path characters in a line horizontally or vertically:<br><br><code>{highlighted_lines}</code>"),
        None => return "This is a valid path.".to_string()
    }
}

fn lines(s: &str) -> Vec<&str> {
    s.lines().collect::<Vec<&str>>()
}

fn max_width(s: &str) -> isize {
    widths(s).max().unwrap()
}

fn width(s: &str) -> isize {
    max_width(s)
}

fn min_width(s: &str) -> isize {
    widths(s).min().unwrap()
}

fn widths(s: &str) -> impl Iterator<Item = isize> + '_ {
    s.lines().map(|l| l.chars().collect::<Vec<char>>().len() as isize)
}

fn height(s: &str) -> isize {
    lines(s).len().try_into().unwrap()
}

fn rectangle(s: &str) -> bool {
    min_width(s) == max_width(s)
}

fn distinct_characters(s: &str) -> Vec<char> {
    s.replace("\n", "").chars().collect::<Vec<char>>().into_iter().unique().collect()
}

fn character_at(l: Location, s: &str) -> char {
    let line = lines(s)[l.y as usize];
    
    line.chars().collect::<Vec<char>>()[l.x as usize]
}

fn character_connecting_opposite_corners(s: &str) -> Option<char> {
    let w = width(s);
    let h = height(s);
    
    let top_left = Location::new(0, 0);
    let bottom_right = Location::new(w - 1, h - 1);
    
    if connected(top_left, bottom_right, s) {
        return Some(character_at(top_left, s));
    }
    
    let top_right = Location::new(w - 1, 0);
    let bottom_left = Location::new(0, h - 1);
    
    if connected(top_right, bottom_left, s) {
        return Some(character_at(top_right, s));
    }
    
    None
}

fn connected(start: Location, finish: Location, s: &str) -> bool {
    let path_character = character_at(start, s);
    let mut potential_path_squares = vec!();
    
    for y in 0..height(s) {
        for x in 0..width(s) {
            let l = Location::new(x, y);
            
            if l != start && character_at(l, s) == path_character {
                potential_path_squares.push(l);
            }
        }
    }
    
    let mut wavefront = vec!(start);

    loop {        
        let mut next_wave = vec!();
        
        for square in wavefront {
            for offset in [Offset::new(-1, 0), Offset::new(1, 0), Offset::new(0, -1), Offset::new(0, 1)] {
                let candidate = square.plus(offset);
                
                
                let candidate_index = potential_path_squares.iter().position(|&s| s == candidate);
                
                match candidate_index {
                    Some(i) => {
                        if candidate == finish {
                            return true;
                        }
                        
                        next_wave.push(candidate);
                        potential_path_squares.swap_remove(i);                        
                    },
                    None => (),
                }
            }
        }
        
        if next_wave.len() == 0 {
            return false;
        }
        
        wavefront = next_wave;
    }
}

fn lines_of_4(path_character: char, s: &str) -> Option<String> {
    let h = height(s);
    let w = width(s);
    
    let mut output_lines = vec!();
    let mut required = false;
    
    for y in 0..h {
        let mut output_line = "".to_string();
        
        for x in 0..w {
            let l = Location::new(x, y);
            let c = character_at(l, s);
            
            if c == path_character && in_a_line_of_4(l, s) {
                required = true;
                output_line = format!("{}<b>{}</b>", output_line, c);
            } else {
                output_line = format!("{}{}", output_line, c);
            }
        }
        
        output_lines.push(output_line);
    }
    
    if required == true {
        return Some(output_lines.join("<br>"));
    } else {
        return None
    }
}

fn in_a_line_of_4(l: Location, s: &str) -> bool {
    same_to_the_left(l, s) + same_to_the_right(l, s) >= 3 || same_above(l, s) + same_below(l, s) >= 3
}

fn same_to_the_left(l: Location, s: &str) -> isize {
    same_with_offset(Offset::new(-1, 0), l, s)
}

fn same_to_the_right(l: Location, s: &str) -> isize {
    same_with_offset(Offset::new(1, 0), l, s)
}

fn same_above(l: Location, s: &str) -> isize {
    same_with_offset(Offset::new(0, -1), l, s)
}

fn same_below(l: Location, s: &str) -> isize {
    same_with_offset(Offset::new(0, 1), l, s)
}

fn same_with_offset(o: Offset, l: Location, s: &str) -> isize {
    let own_character = character_at(l, s);
    let mut count = 0;
    
    for n in 1..=3 {
        let candidate = l.plus(o.times(n));
        if within_grid(candidate, s) && character_at(candidate, s) == own_character {
            count += 1;
        } else {
            break;
        }
    }
    
    count
}

fn within_grid(l: Location, s: &str) -> bool {
    l.x >= 0 && l.y >= 0 && l.x < width(s) && l.y < height(s)
}


use crate::*;
use std::collections::HashMap;

fn discretise25(point: &Point, x: f32, y: f32) -> char {
    let mat: [[char; 5]; 5] = [
        ['a', 'b', 'c', 'd', 'e'],
        ['f', 'g', 'h', 'i', 'j'],
        ['k', 'l', 'm', 'n', 'o'],
        ['p', 'q', 'r', 's', 't'],
        ['u', 'v', 'w', 'x', 'y'],
    ];

    let x = ((point.x - (52. * x)) / 10.4) as usize;
    let y = ((point.y - (52. * y)) / 10.4) as usize;

    mat[y][x]
}

fn discretise9(point: &Point, x: f32, y: f32) -> char {
    let mat: [[char; 3]; 3] = [['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', 'i']];

    let x = ((point.x - (52. * x)) / 17.33) as usize;
    let y = ((point.y - (52. * y)) / 17.33) as usize;

    mat[y][x]
}

fn discretise4d(point: &Point, x: f32, y: f32) -> char {
    let mat: [[char; 4]; 8] = [
        ['a', 'b', 'c', 'd'],
        ['d', 'e', 'f', 'g'],
        ['h', 'i', 'j', 'k'],
        ['l', 'm', 'n', 'o'],
        ['p', 'q', 'r', 's'],
        ['t', 'u', 'v', 'w'],
        ['x', 'y', 'z', '0'],
        ['1', '2', '3', '4'],
    ];

    let x = ((point.x - (52. * x)) / 26.) as usize;
    let y = ((point.y - (52. * y)) / 26.) as usize;
    let q = if x == 0 {
        if y == 0 {
            0
        } else {
            1
        }
    } else {
        if y == 0 {
            2
        } else {
            3
        }
    };
    //    let d = (point.direction / 0.785398) as usize;
    let d = (point.direction / 1.5707) as usize;

    println!("{:?} - {:?}", point.direction, d);
    mat[d][q]
}

struct Glyph<'a> {
    strokes: Vec<&'a Line>,
    cell: (usize, usize),
}

impl Glyph<'_> {
    fn read(self) -> Vec<char> {
        let mut dstrokes: Vec<char> = vec![];
        for stroke in self.strokes {
            let mut last = 'z';
            for i in 0..stroke.points.len() {
                let p = &stroke.points[i];
                let chr = discretise4d(p, self.cell.0 as f32, self.cell.1 as f32);
                if chr != last {
                    last = chr;
                    dstrokes.push(chr);
                };
            }
            dstrokes.push('_');
        }
        dstrokes
    }
}

fn id_cell(x: f32, y: f32) -> (usize, usize) {
    let s = 52.;
    ((x / s) as usize, (y / s) as usize)
}

pub fn read_page(page: &Page) -> HashMap<(usize, usize), Vec<char>> {
    let mut cellstrokes = HashMap::new();
    for layer in &page.layers {
        for line in &layer.lines {
            let mut x_start = 10000;
            let mut x_end = 0;
            let mut y_start = 10000;
            let mut y_end = 0;

            if line.points.is_empty() {
                continue;
            }

            for point in &line.points {
                x_start = std::cmp::min(x_start, point.x as u32);
                x_end = std::cmp::max(x_end, point.x as u32);
                y_start = std::cmp::min(y_start, point.y as u32);
                y_end = std::cmp::max(y_end, point.y as u32);
            }
            let start = (x_start, y_start);
            let end = (x_end, y_end);
            // group all strokes that are contained by a cell together
            //let centre = (x_end - x_start / 2, y_end - y_start / 2);
            let cell = id_cell(x_start as f32, y_start as f32);
            if cell == id_cell(x_end as f32, y_end as f32) {
                //println!("adding stroke {:?}:{:?}\t{:?}", start, end, cell);
                cellstrokes.entry(cell).or_insert(Vec::new()).push(line);
                //                cellstrokes.insert(id_cell(x_start, x_end))
            } else {
                if x_end - x_start > 100 && y_end - y_start > 60 {
                    println!("large shape detected");
                }
                println!(
                    "colouring outside the lines {:?}:{:?}\t{:?}",
                    start, end, cell
                );
            }
            //            let (row, col) = id_cell(&line.points[0]);
            //            if row < 27 && col < 36 {
            //                let g = Glyph { start: Point { x }
            //            }
            // }
        }
    }
    let mut glyphs = HashMap::new();
    for (cell, strokes) in cellstrokes.iter() {
        let g = Glyph {
            strokes: strokes.to_vec(),
            cell: *cell,
        };
        //        println!("{:?}:\t{:?}", cell, g.read());
        glyphs.insert(*cell, g.read());
    }
    glyphs
}

// reading
// 1. tree of bounding boxes (2d interval tree). sort into size classes. store order
// 2. group coincident strokes. union of overlapping boxes
// 3. centre cell, discritise path

// training:
// read rows into trie

// path recognition:
// identify strokes that link larger features (box diagrams)

//pub fn train_strokes(page: &Page) -> Hashmap<[u8], u8> {

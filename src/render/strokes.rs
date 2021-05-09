use crate::*;

fn discretise(point: &Point, x: f32, y: f32) -> char {
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

struct Glyph<'a> {
    strokes: Vec<&'a Line>,
    origin: Point,
    centre: Point,
}

impl Glyph<'_> {
    fn read(self) -> Vec<char> {
        let mut dstrokes: Vec<char> = vec![];
        for stroke in self.strokes {
            let mut last = 'z';
            for i in 0..stroke.points.len() {
                let p = &stroke.points[i];
                let chr = discretise(p, self.origin.x, self.origin.y);
                if chr != last {
                    last = chr;
                    dstrokes.push(chr);
                };
            }
        }
        vec!['a']
    }
}

fn id_cell(point: &Point) -> (usize, usize) {
    let s = 52.;
    ((point.x / s) as usize, (point.y / s) as usize)
}

pub fn read_page(page: &Page) -> u32 {
    for layer in &page.layers {
        for line in &layer.lines {
            let mut x_start = 1000;
            let mut x_end = 0;
            let mut y_start = 1000;
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
            let centre = (x_end - x_start / 2, y_end - y_start / 2);
            let (row, col) = id_cell(&line.points[0]);
            if row < 27 && col < 36 {
                println!("{}:{}\t{:?}", row, col, centre);
            }
            // }
        }
    }
    0
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

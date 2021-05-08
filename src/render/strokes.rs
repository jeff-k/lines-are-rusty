use crate::*;

fn id_cell(point: &Point) -> (usize, usize) {
    let s = 52.;
    ((point.x / s) as usize, (point.y / s) as usize)
}

fn c(point: &Point, x: f32, y: f32) -> char {
    let mat: [[char; 5]; 5] = [
        ['a', 'b', 'c', 'd', 'e'],
        ['f', 'g', 'h', 'i', 'j'],
        ['k', 'l', 'm', 'n', 'o'],
        ['p', 'q', 'r', 's', 't'],
        ['u', 'v', 'w', 'x', 'y'],
    ];

    let x = ((point.x - (52. * x)) / 10.4) as usize;
    let y = ((point.y - (52. * y)) / 10.4) as usize;

    mat[x][y]
}

pub fn read_page(page: &Page) -> u32 {
    for layer in &page.layers {
        for line in &layer.lines {
            if line.points.is_empty() {
                continue;
            }
            let (row, col) = id_cell(&line.points[0]);
            if row < 27 && col < 36 {
                let mut strokes: Vec<char> = vec![];
                let mut last = 'z';
                for i in 0..line.points.len() {
                    let p = &line.points[i];
                    let chr = c(p, row as f32, col as f32);
                    if chr != last {
                        last = chr;
                        strokes.push(chr);
                    };
                }
                println!("{}:{}\t{:?}", row, col, strokes);
            }
        }
    }
    0
}

//pub fn train_strokes(page: &Page) -> Hashmap<[u8], u8> {

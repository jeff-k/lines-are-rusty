use crate::*;
use bresenham::Bresenham;

fn id_cell(point: &Point) -> (usize, usize) {
    let s = 52.;
    ((point.x / s) as usize, (point.y / s) as usize)
}

fn c(point: &Point, x: f32, y: f32) -> (isize, isize) {
    let x = (point.x - (52. * x)) / 2.;
    let y = (point.y - (52. * y)) / 2.;

    (x as isize, y as isize)
}

pub fn render_bitmap(pages: &[Page]) {
    // One page per .rm file(?)
    let page = &pages[0];
    let pd = read_page(page);
    print_cells(vec![pd[2][17], pd[3][14], pd[5][21], pd[8][21]]);
}

pub fn read_page(page: &Page) -> [[[[u8; 26]; 26]; 36]; 27] {
    let mut image = [[[[0; 26]; 26]; 36]; 27];
    for layer in &page.layers {
        for line in &layer.lines {
            if line.points.is_empty() {
                // what condition is this?
                continue;
            }
            let (row, col) = id_cell(&line.points[0]);
            if row < 27 && col < 36 {
                image[row][col] = get_sq(row, col, 0, 0, line);
            }
        }
    }
    image
}

pub fn get_sq(
    row: usize,
    col: usize,
    x_offset: isize,
    y_offset: isize,
    line: &Line,
) -> [[u8; 26]; 26] {
    let mut image = [[0; 26]; 26];
    for i in 1..line.points.len() {
        let p1 = &line.points[i - 1];
        let p2 = &line.points[i];
        let (x1, y1) = c(p1, row as f32, col as f32);
        let (x2, y2) = c(p2, row as f32, col as f32);
        for (x, y) in Bresenham::new((x1, y1), (x2, y2)) {
            let x_ = x + x_offset;
            let y_ = y + y_offset;
            if x_ < 26 && y_ < 26 && x_ >= 0 && y_ >= 0 {
                image[y_ as usize][x_ as usize] = 255;
            }
        }
    }
    image
    //    let x: [f32] = image.iter().flat_map(|array| array.iter()).collect();
}

pub fn print_cells(cells: Vec<[[u8; 26]; 26]>) {
    for row in 0..26 {
        for cell in cells.iter() {
            for pixel in cell[row].iter() {
                if *pixel == 255 as u8 {
                    print!("X");
                } else {
                    print!(" ");
                };
            }
        }
        println!();
    }
}

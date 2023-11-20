use std::io;
use std::collections::HashSet;

#[derive(Clone)]
struct Cell {
    value: i32,
    value_options: HashSet<i32>
}

impl Default for Cell {
    fn default() -> Cell { 
        Cell{
            value: 0,
            value_options: HashSet::from([1,2,3,4,5,6,7,8,9])
        }
    }
}


struct Grid {
    cells: Vec<Cell>,
    cells_solved: i32
}

impl Default for Grid {
    fn default() -> Grid {
        let mut cell_vector: Vec<Cell> = Vec::<Cell>::new();
        for _i in 0..81 {
            cell_vector.push(Cell::default())
        }
        Grid{
            cells: cell_vector,
            cells_solved: 0
        }
    }
}

impl Grid {
    fn calc_cell_accessor(row: usize, col: usize) -> usize {
        row*9 + col
    }

    fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[Grid::calc_cell_accessor(row, col)].clone()
    }

    fn set_cell(&mut self, row: usize, col: usize, cell: Cell) {
        if self.cells[Grid::calc_cell_accessor(row,col)].value != 0 {
            return 
        }
        if cell.value != 0 {
            self.cells_solved += 1
        }
        self.cells[Grid::calc_cell_accessor(row,col)] = cell;
    }

    fn add_solved(&mut self) {
        self.cells_solved +=1;
    }

    fn solved(&self) -> bool {
        return self.cells_solved == 81;
    }

    fn get_row(&self, row: usize) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::<&Cell>::new();
        for col in 0..9 {
            let cell = &self.cells[Grid::calc_cell_accessor(row, col)];
            if cell.value != 0 {
                cells.push(cell);
            }
        }
        return cells;
    }

    fn get_col(&self, col: usize) -> Vec<&Cell> {
        let mut cells: Vec<&Cell> = Vec::<&Cell>::new();
        for row in 0..9 {
            let cell = &self.cells[Grid::calc_cell_accessor(row, col)];
            if cell.value != 0 {
                cells.push(cell);
            }

        }
        return cells;
    }

    fn get_square(&self, row: usize, col: usize) -> Vec<&Cell> {
        let square_row = row / 3;
        let square_col = col / 3;
        let mut cells: Vec<&Cell> = Vec::<&Cell>::new();
        for row_index in square_row*3..square_row*3+3 {
            for col_index in square_col*3..square_col*3+3{
                let cell = &self.cells[Grid::calc_cell_accessor(row_index, col_index)];
                if cell.value != 0 {
                    cells.push(cell);
                }
            }
        }
        return cells;
    }
}

fn print_grid (grid: &Grid ) {
    println!("Solved Cells: {0}", grid.cells_solved);
    for row in 0..9{
        for col in 0..9 {
            let cell = grid.get_cell(row, col);
            if cell.value > 0 {
                print!("{0},",cell.value);
            } else {
                print!("!,");
            }
        }
        println!();
    }
}

fn compute_cell(grid: &Grid, row: usize, col:usize) -> Cell {

    let mut cell = grid.get_cell(row, col);
    if cell.value != 0 {
        return cell;
    }

    let row_cells = grid.get_row(row);
    for row_cell_index in 0..row_cells.len() {
        cell.value_options.remove(&row_cells[row_cell_index].value);
    }

    let col_cells = grid.get_col(col);
    for col_cell_index in 0..col_cells.len(){
        cell.value_options.remove(&col_cells[col_cell_index].value);
    }

    let square_cells = grid.get_square(row, col);
    for square_cells_index in 0..square_cells.len(){
        cell.value_options.remove(&square_cells[square_cells_index].value);
    }

    if cell.value_options.len() == 1 {
        cell.value = *cell.value_options.iter().last().unwrap_or(&0i32);
        //grid.add_solved()
    }
    return cell
}

fn main() {
    let mut grid: Grid = Grid::default();
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(io::stdin());

    let mut row = 0;
    for result in rdr.records() {
        let mut col = 0;
        let record = result.expect("a CSV record");
        for i in 0..record.len() {
            let field = &record[i];
            if !field.is_empty() {
                grid.set_cell(row, col, Cell{value: field.parse().unwrap(), value_options: HashSet::new()})
            }
            col += 1;
        }
        row += 1;
    }
    println!("Starting Grid");
    println!("-------------");
    print_grid(&grid);
    println!("-------------");

    //brute force solve the grid
    let mut loop_index = 0;
    while !grid.solved() {
        for row in 0..9 {
            for col in 0..9{
                let new_cell = compute_cell(&grid, row, col);
                grid.set_cell(row, col, new_cell);
            }
        }
        println!("Loop: {0}, Solved Cells: {1}", loop_index, grid.cells_solved);
        loop_index += 1;
    }
    println!("-------------");
    println!("Sudoku Solved");
    println!("-------------");
    print_grid(&grid);
}

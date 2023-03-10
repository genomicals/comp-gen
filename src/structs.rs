/// Multi-dimensional contiguous matrix
#[derive(Debug)]
pub struct Matrix<T: Default + Clone> {
    matrix: Vec<T>,
    y: usize,
}
impl<T: Default + Clone> Matrix<T> {

    /// Create a new matrix with the given dimensions
    pub fn with_shape(x: usize, y: usize) -> Self {
        Matrix {
            matrix: vec![T::default(); x*y],
            y,
        }
    }

    /// Returns an immutable reference to item at the given index
    pub fn index(&self, x: usize, y: usize) -> &T {
        &self.matrix[self.y*x+y]
    }

    /// Returns an immutable reference to item at the given index
    pub fn index_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.matrix[self.y*x+y]
    }
}


/// Makes up one cell of a table
#[derive(Debug, Clone)]
pub struct Cell {
    pub d_score: i32,
    pub i_score: i32,   
    pub s_score: i32,
}
impl Cell {
    pub fn new() -> Cell {
        Cell {
            d_score: 0,
            i_score: 0,
            s_score: 0,
        }
    }

    /// Super-optimized comparison maxxing algorithm
    pub fn score(&self) -> i32 {
        if self.d_score > self.i_score {
            if self.d_score > self.s_score {
                self.d_score
            } else {
                self.s_score
            }
        } else {
            if self.i_score > self.s_score {
                self.i_score
            } else {
                self.s_score
            }
        }
    }
}
impl Default for Cell {
    fn default() -> Self {
        Cell::new()
    }
}


/// Used to keep config settings in one place
#[derive(Debug)]
pub struct Config {
    pub true_match: i32,
    pub mismatch: i32,
    pub h: i32,
    pub g: i32,
    pub s1_name: String,
    pub s2_name: String,
}


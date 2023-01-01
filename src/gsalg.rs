#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub columns: usize,
    elements: Vec<f64>
}
impl Matrix {
    pub fn new_zero(rows: usize, columns: usize) -> Self {
        Self::new(rows, columns, (0..rows*columns).map(|_| 0.0).collect())
    }
    pub fn new(rows: usize, columns: usize, elements: Vec<f64>) -> Self {
        Self {
            rows,
            columns,
            elements
        }
    }
    pub fn size(&self) -> usize {
        self.rows * self.columns
    }
    pub fn get(&self, row: usize, column: usize) -> f64 {
        self.elements[self.columns * row + column]
    }
    pub fn set(&mut self, row: usize, column: usize, value: f64) {
        self.elements[self.columns * row + column] = value;
    }
    pub fn binary_element_operation(a: &Self, b: &Self, func: fn(x: f64, y: f64) -> f64)
        -> Result<Self, &'static str> {
        if a.size() != b.size() {return Err("Matrix sizes must match");}
        let new_elements: Vec<f64> = (0..a.size())
            .map(|x| func(a.elements[x], b.elements[x])).collect();
        Ok(Self {
            rows: a.rows,
            columns: a.columns,
            elements: new_elements
        })
    }
    pub fn add(a: &Self, b: &Self) -> Result<Self, &'static str> {Self::binary_element_operation(a, b, |a, b| a + b)}
    pub fn sub(a: &Self, b: &Self) -> Result<Self, &'static str> {Self::binary_element_operation(a, b, |a, b| a - b)}
    pub fn mul(a: &Self, b: &Self) -> Result<Self, &'static str> {Self::binary_element_operation(a, b, |a, b| a * b)}
    pub fn div(a: &Self, b: &Self) -> Result<Self, &'static str> {Self::binary_element_operation(a, b, |a, b| a / b)}
    pub fn matmul(a: &Self, b: &Self) -> Result<Self, &'static str> {
        if a.columns != b.rows {
            return Err("Columns of a must match rows of b");
        }
        let mut new_matrix = Self::new_zero(a.rows, b.columns);
        for i in 0..new_matrix.rows {
            for j in 0..new_matrix.columns {
                let value = (0..a.columns).map(|k| a.get(i, k) * b.get(k, j)).sum();
                new_matrix.set(i, j, value);
            }
        }
        Ok(new_matrix)
    }
}

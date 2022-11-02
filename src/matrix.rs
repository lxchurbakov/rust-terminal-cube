#[derive(Debug, PartialEq, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.cols, rhs.rows);

        let rows = self.rows;
        let cols = rhs.cols;
        let size = self.cols;    

        let mut result = Matrix { rows, cols, data: Vec::new() };

        for y in 0..rows {
            for x in 0..cols {
                let mut sum = 0.0;

                for i in 0..size {
                    sum += self.data[i + y * size] * rhs.data[i * size + x];
                }

                result.data.push(sum);
            }
        }

        result
    }
}

#[macro_export]
macro_rules! matrix {
    [ $($($x:expr),*);* ] => {
        {
            let mut m = Matrix { cols: 0, rows: 0, data: Vec::new() };
            m.rows = 0;

            $(
                m.rows += 1;
                $(
                    m.data.push($x as f64);
                )*
            )*
            
            if m.rows > 0 {
                m.rows -= 1;
            }

            m.cols = m.data.len() / m.rows;

            m
        }
    };
}

// TODO add tests
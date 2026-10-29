use pyo3::pymodule;

#[pymodule]
mod exmath {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    #[pyfunction]
    fn even(n: usize) -> bool {
        n % 2 == 0
    }

    #[pyfunction]
    fn odd(n: usize) -> bool {
        n % 2 != 0
    }

    #[pyfunction]
    fn digit(n: u8) -> PyResult<char> {
        const BASE36_ALPHABET: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

        if n > 35 {
            return Err(PyValueError::new_err("digit must be <= 35"));
        }

        Ok(BASE36_ALPHABET[n as usize] as char)
    }

    #[pyfunction]
    fn base(mut n: usize, base: u8) -> PyResult<String> {
        if !(2..=36).contains(&base) {
            return Err(PyValueError::new_err("base must be >= 2 and <= 36"));
        }

        let mut digits: Vec<char> = Vec::with_capacity(usize::BITS as usize);
        let base = base as usize;
        while n >= base {
            digits.push(digit((n % base) as u8)?);
            n /= base;
        }
        digits.push(digit(n as u8)?);
        digits.reverse();

        Ok(digits.into_iter().collect())
    }

    #[pymodule]
    mod trigonometry {
        use pyo3::prelude::*;
        use std::f64::consts::PI;

        #[pyfunction]
        fn deg_to_grad(deg: f64) -> f64 {
            deg * 10.0 / 9.0
        }

        #[pyfunction]
        fn grad_to_deg(grad: f64) -> f64 {
            grad * 0.9
        }

        #[pyfunction]
        fn rad_to_grad(rad: f64) -> f64 {
            rad * 200.0 / PI
        }

        #[pyfunction]
        fn grad_to_rad(grad: f64) -> f64 {
            grad * PI / 200.0
        }
    }
}

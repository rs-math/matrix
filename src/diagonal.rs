use std::ops::{Deref, DerefMut};

use {Band, Compressed, Dense, Element, Major, Make, Shape, Sparse};

/// A diagonal matrix.
///
/// The storage is suitable for generic diagonal matrices.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagonal<T: Element> {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The values of the diagonal elements.
    pub values: Vec<T>,
}

macro_rules! debug_valid(
    ($matrix:ident) => (debug_assert!(
        $matrix.values.len() == min!($matrix.rows, $matrix.columns)
    ));
);

matrix!(Diagonal);

impl<'l, T: Element> Make<&'l [T]> for Diagonal<T> {
    fn make(values: &'l [T], shape: Shape) -> Self {
        let (rows, columns) = match shape {
            Shape::Square(size) => (size, size),
            Shape::Rectangular(rows, columns) => (rows, columns),
        };
        debug_assert_eq!(values.len(), min!(rows, columns));
        Diagonal { rows: rows, columns: columns, values: values.to_vec() }
    }
}

impl<T: Element> Make<Vec<T>> for Diagonal<T> {
    fn make(values: Vec<T>, shape: Shape) -> Self {
        let (rows, columns) = match shape {
            Shape::Square(size) => (size, size),
            Shape::Rectangular(rows, columns) => (rows, columns),
        };
        debug_assert_eq!(values.len(), min!(rows, columns));
        Diagonal { rows: rows, columns: columns, values: values }
    }
}

impl<T: Element> Sparse for Diagonal<T> {
    #[inline]
    fn nonzeros(&self) -> usize {
        if self.rows < self.columns { self.rows } else { self.columns }
    }
}

impl<'l, T: Element> From<&'l Diagonal<T>> for Band<T> {
    #[inline]
    fn from(diagonal: &'l Diagonal<T>) -> Band<T> {
        diagonal.clone().into()
    }
}

impl<T: Element> From<Diagonal<T>> for Band<T> {
    fn from(diagonal: Diagonal<T>) -> Band<T> {
        debug_valid!(diagonal);
        Band {
            rows: diagonal.rows,
            columns: diagonal.columns,
            superdiagonals: 0,
            subdiagonals: 0,
            values: {
                let mut values = diagonal.values;
                for _ in diagonal.rows..diagonal.columns {
                    values.push(T::zero());
                }
                values
            },
        }
    }
}

impl<'l, T: Element> From<&'l Diagonal<T>> for Compressed<T> {
    #[inline]
    fn from(diagonal: &'l Diagonal<T>) -> Compressed<T> {
        diagonal.clone().into()
    }
}

impl<T: Element> From<Diagonal<T>> for Compressed<T> {
    #[inline]
    fn from(diagonal: Diagonal<T>) -> Compressed<T> {
        debug_valid!(diagonal);
        let Diagonal { rows, columns, values } = diagonal;
        let nonzeros = values.len();
        Compressed {
            rows: rows,
            columns: columns,
            nonzeros: nonzeros,
            values: values,
            format: Major::Column,
            indices: (0..nonzeros).collect(),
            offsets: (0..(nonzeros + 1)).collect(),
        }
    }
}

impl<'l, T: Element> From<&'l Diagonal<T>> for Dense<T> {
    #[inline]
    fn from(diagonal: &Diagonal<T>) -> Dense<T> {
        debug_valid!(diagonal);

        let &Diagonal { rows, columns, ref values } = diagonal;

        let mut dense = Dense {
            rows: rows,
            columns: columns,
            values: vec![T::zero(); rows * columns],
        };

        for i in 0..min!(rows, columns) {
            dense.values[i * rows + i] = values[i];
        }

        dense
    }
}

impl<T: Element> From<Diagonal<T>> for Dense<T> {
    #[inline]
    fn from(diagonal: Diagonal<T>) -> Dense<T> {
        (&diagonal).into()
    }
}

impl<T: Element> Into<Vec<T>> for Diagonal<T> {
    #[inline]
    fn into(self) -> Vec<T> {
        self.values
    }
}

impl<T: Element> Deref for Diagonal<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        self.values.deref()
    }
}

impl<T: Element> DerefMut for Diagonal<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut [T] {
        self.values.deref_mut()
    }
}

#[cfg(test)]
mod tests {
    use {Band, Compressed, Dense, Diagonal, Major};

    #[test]
    fn into_band_tall() {
        let diagonal = Diagonal { rows: 5, columns: 3, values: vec![1.0, 2.0, 3.0] };
        let band: Band<_> = diagonal.into();
        assert_eq!(&band.values, &[1.0, 2.0, 3.0]);
    }

    #[test]
    fn into_band_wide() {
        let diagonal = Diagonal { rows: 3, columns: 5, values: vec![1.0, 2.0, 3.0] };
        let band: Band<_> = diagonal.into();
        assert_eq!(&band.values, &[1.0, 2.0, 3.0, 0.0, 0.0]);
    }

    #[test]
    fn into_compressed() {
        let diagonal = Diagonal { rows: 5, columns: 3, values: vec![1.0, 2.0, 0.0] };

        let compressed: Compressed<_> = diagonal.into();

        assert_eq!(compressed, Compressed {
            rows: 5, columns: 3, nonzeros: 3, format: Major::Column, values: vec![1.0, 2.0, 0.0],
            indices: vec![0, 1, 2], offsets: vec![0, 1, 2, 3]
        });
    }

    #[test]
    fn into_dense() {
        let diagonal = Diagonal { rows: 3, columns: 5, values: vec![1.0, 2.0, 3.0] };

        let dense: Dense<_> = diagonal.into();

        assert_eq!(dense.rows, 3);
        assert_eq!(dense.columns, 5);
        assert_eq!(&dense.values, &[
            1.0, 0.0, 0.0,
            0.0, 2.0, 0.0,
            0.0, 0.0, 3.0,
            0.0, 0.0, 0.0,
            0.0, 0.0, 0.0,
        ]);
    }
}

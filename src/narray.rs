// TODO: implement generic length vector datatype
// Using array3.rs for now (fixed length)
pub mod narray {
    use num_traits::Num;

    use std::ops::{Add, Sub, Div, Mul, AddAssign, SubAssign, DivAssign, MulAssign };

    use std::iter::zip;
    use std::convert::TryInto;

    pub struct NArray<T, const N: usize>
    where
        T: Num
    {
        elems: [T; N],
    }

    //Add 
    impl <T, const N: usize> Add for NArray<T, N>
    where
        T: Num
    {
        type Output = NArray<T, N>;

        fn add(self, rhs: NArray<T, N>) -> Self::Output {
            let new_elems: [T; N] = self.elems.iter().zip(rhs.elems.iter())
                                    .map(|(a, b): (&T, &T)| -> T { *a + *b })
                                    .collect::<Vec<T>>()
                                    .try_into()
                                    .unwrap_or_else(
                                        |v: Vec<T>| panic!("Expected a Vec of length {}, got length {}", N, v.len())
                                    );
            NArray { elems: new_elems, }
        }
    }
}

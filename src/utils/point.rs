use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Mul, Neg, Sub},
};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, PartialOrd, Debug, Clone, Copy)]
pub struct Pt<const DIMS: usize>(pub [isize; DIMS]);

impl<const DIMS: usize> Default for Pt<DIMS> {
    fn default() -> Self {
        Self([0; DIMS])
    }
}

impl<const DIMS: usize> Neg for Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn neg(mut self) -> Self::Output {
        for v in &mut self.0 {
            *v *= -1
        }
        self
    }
}

impl<const DIMS: usize> Sub for Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for (i, a) in &mut self.0.iter_mut().enumerate() {
            *a -= rhs.0[i]
        }
        self
    }
}

impl<'a, const DIMS: usize> Sub<&'a Pt<DIMS>> for &Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn sub(self, rhs: &'a Pt<DIMS>) -> Self::Output {
        let mut subbed = [0; DIMS];
        for (i, element) in subbed.iter_mut().enumerate().take(DIMS) {
            *element = self.0[i] - rhs.0[i];
        }
        Pt(subbed)
    }
}

impl<const DIMS: usize> Add for Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (i, a) in &mut self.0.iter_mut().enumerate() {
            *a += rhs.0[i]
        }
        self
    }
}

impl<'a, const DIMS: usize> Add<&'a Pt<DIMS>> for &Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn add(self, rhs: &'a Pt<DIMS>) -> Self::Output {
        let mut added = [0; DIMS];
        for (i, element) in added.iter_mut().enumerate().take(DIMS) {
            *element = self.0[i] + rhs.0[i];
        }
        Pt(added)
    }
}
impl<const DIMS: usize> AddAssign for Pt<DIMS> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const DIMS: usize> Mul<isize> for Pt<DIMS> {
    type Output = Pt<DIMS>;

    fn mul(mut self, rhs: isize) -> Self::Output {
        for v in &mut self.0 {
            *v *= rhs;
        }
        self
    }
}

#[allow(dead_code)]
impl<const DIMS: usize> Pt<DIMS> {
    /// get all the offsets required to get every neighbour to a position
    pub fn neighbour_offsets() -> HashSet<Pt<DIMS>> {
        vec![[-1, 0, 1]; DIMS]
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| vec.try_into().unwrap())
            .filter(|arr| arr != &[0; DIMS])
            .map(Pt)
            .collect()
    }

    /// get all the offsets required to get every cardinal (non-diagonal) neighbour to a position
    pub fn card_offsets() -> HashSet<Pt<DIMS>> {
        let mut pts = vec![[0; DIMS]; DIMS * 2];

        for i in 0..DIMS {
            pts[i][i] = 1;
            pts[i + DIMS][i] = -1;
        }

        pts.into_iter().map(Pt).collect()
    }

    pub fn mag(&self) -> isize {
        self.0.iter().map(|v| v.abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Pt;
    use rstest::rstest;

    #[test]
    fn validate_offsets() {
        let expected_2d: HashSet<Pt<2>> = vec![
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_2d, Pt::<2>::neighbour_offsets());

        let expected_3d: HashSet<Pt<3>> = vec![
            [-1, -1, -1],
            [-1, -1, 0],
            [-1, -1, 1],
            [-1, 0, -1],
            [-1, 0, 0],
            [-1, 0, 1],
            [-1, 1, -1],
            [-1, 1, 0],
            [-1, 1, 1],
            [0, -1, -1],
            [0, -1, 0],
            [0, -1, 1],
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, -1],
            [0, 1, 0],
            [0, 1, 1],
            [1, -1, -1],
            [1, -1, 0],
            [1, -1, 1],
            [1, 0, -1],
            [1, 0, 0],
            [1, 0, 1],
            [1, 1, -1],
            [1, 1, 0],
            [1, 1, 1],
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_3d, Pt::<3>::neighbour_offsets());
    }

    #[test]
    fn validate_card_offsets() {
        let expected_2d: HashSet<Pt<2>> = vec![[-1, 0], [0, -1], [0, 1], [1, 0]]
            .into_iter()
            .map(Pt)
            .collect();

        assert_eq!(expected_2d, Pt::<2>::card_offsets());

        let expected_3d: HashSet<Pt<3>> = vec![
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_3d, Pt::<3>::card_offsets());
    }

    #[rstest]
    #[case(Pt([1,2,3,4]), Pt([1,2,0,0]), Pt([0,0,3,4]))]
    #[case(Pt([-102,34,0,-3]), Pt([100,14,-10000,999]), Pt([-202,20,10000,-1002]))]
    fn validate_add(#[case] expected: Pt<4>, #[case] a: Pt<4>, #[case] b: Pt<4>) {
        assert_eq!(expected, a + b)
    }

    #[rstest]
    #[case(Pt([100,200,300]), Pt([1,2,3]), 100)]
    #[case(Pt([-100,-200,-300]), Pt([1,2,3]), -100)]
    fn validate_mul(#[case] expected: Pt<3>, #[case] a: Pt<3>, #[case] b: isize) {
        assert_eq!(expected, a * b)
    }
}

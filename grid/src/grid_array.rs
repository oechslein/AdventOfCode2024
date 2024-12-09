//! Grid based on a vector

use std::fmt::Display;
use std::mem::{replace, swap};

use itertools::Itertools;

use derive_builder::Builder;

use crate::grid_iteration::{adjacent_cell, all_adjacent_directions, is_corner, is_edge};
use crate::grid_types::Direction;

use super::grid_iteration;
use super::grid_types::{Neighborhood, Topology, UCoor2D, UCoor2DIndex};

/// `GridArray`
#[allow(missing_docs, unused_mut)]
#[derive(Builder, Clone, Debug, Hash, Eq, PartialEq)]
pub struct GridArray<T: Default + Clone + std::fmt::Display> {
    /// width of the grid
    width: UCoor2DIndex,
    height: UCoor2DIndex,

    #[builder(default = "Topology::Bounded")]
    topology: Topology,
    #[builder(default = "Neighborhood::Square")]
    neighborhood: Neighborhood,

    //    #[builder(setter(skip), default = "self.create_data_vec()")]
    #[builder(default = "self.create_data_vec()")]
    data: Vec<T>,
}

impl<T: Default + Clone + std::fmt::Display> GridArrayBuilder<T> {
    fn create_data_vec(&self) -> Vec<T> {
        vec![T::default(); self.width.unwrap() * self.height.unwrap()]
    }
}

impl<T: Default + Clone + std::fmt::Display> Display for GridArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.get_unchecked(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl GridArray<char> {
    /// from newline separated string
    /// #Panics panics if the string is not a rectangle
    pub fn from_newline_separated_string(
        topology: Topology,
        neighborhood: Neighborhood,
        input: &str,
    ) -> Self {
        let width = input
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n' || *x == '\r')
            .take(1)
            .next()
            .unwrap()
            .0;
        let data = input
            .chars()
            .filter(|x| *x != '\n' && *x != '\r')
            .collect_vec();
        GridArray::from_1d_vec(topology, neighborhood, width, data)
    }
}

impl<T: Default + Clone + std::fmt::Display> GridArray<T> {
    /// from newline separated string
    /// #Panics panics if the string is not a rectangle
    pub fn from_newline_separated_string_into(
        topology: Topology,
        neighborhood: Neighborhood,
        input: &str,
        mapping_fn: impl Fn(char) -> T,
    ) -> Self {
        let width = input
            .chars()
            .enumerate()
            .filter(|(_, x)| *x == '\n' || *x == '\r')
            .take(1)
            .next()
            .unwrap()
            .0;
        let data = input
            .chars()
            .filter(|x| *x != '\n' && *x != '\r')
            .map(mapping_fn)
            .collect_vec();
        GridArray::from_1d_vec(topology, neighborhood, width, data)
    }
}

impl<T: Default + Clone + std::fmt::Display> GridArray<T> {
    #[allow(dead_code)]
    fn create_data_vec(&self) -> Vec<T> {
        vec![T::default(); self.width * self.height]
    }

    /// from 1d vector
    pub fn from_1d_vec(
        topology: Topology,
        neighborhood: Neighborhood,
        width: UCoor2DIndex,
        data: Vec<T>,
    ) -> Self {
        debug_assert_eq!(
            (data.len()) % width,
            0,
            "data.len()={} width={}",
            data.len(),
            width
        );
        GridArray {
            width,
            height: data.len() / width,
            topology,
            neighborhood,
            data,
        }
    }

    /// from 2d vector
    pub fn from_2d_vec(topology: Topology, neighborhood: Neighborhood, data: Vec<Vec<T>>) -> Self {
        debug_assert!(!data.is_empty());
        GridArray {
            width: data[0].len(),
            height: data.len(),
            topology,
            neighborhood,
            data: data.into_iter().flatten().collect(),
        }
    }

    #[allow(unused_comparisons)]
    fn _check_index(x: UCoor2DIndex, y: UCoor2DIndex, width: usize, height: usize) -> bool {
        #![allow(clippy::absurd_extreme_comparisons)]
        (0 <= x && x < width) && (0 <= y && y < height)
    }

    #[allow(unused_comparisons)]
    fn check_index(&self, x: UCoor2DIndex, y: UCoor2DIndex) -> bool {
        GridArray::<T>::_check_index(x, y, self.width, self.height)
    }

    fn _index_to_vec_index(x: usize, y: usize, width: usize) -> usize {
        y * width + x
    }

    fn index_to_vec_index(&self, x: usize, y: usize) -> usize {
        debug_assert!(
            self.check_index(x, y),
            "x={} y={} width={} height={}",
            x,
            y,
            self.width,
            self.height
        );
        GridArray::<T>::_index_to_vec_index(x, y, self.width)
    }

    /// `get_width`
    pub fn width(&self) -> usize {
        self.width
    }

    /// `get_height`
    pub fn height(&self) -> usize {
        self.height
    }

    /// `get_topology`
    pub fn get_topology(&self) -> Topology {
        self.topology
    }

    /// `get_neighborhood`
    pub fn get_neighborhood(&self) -> Neighborhood {
        self.neighborhood
    }

    /// `all_adjacent_directions`
    pub fn all_adjacent_directions(&self) -> impl Iterator<Item = Direction> {
        all_adjacent_directions(self.neighborhood)
    }

    /// `is_edge`
    pub fn is_edge(&self, x: UCoor2DIndex, y: UCoor2DIndex) -> bool {
        debug_assert!(self.check_index(x, y));
        is_edge(self.topology, self.width, self.height, &UCoor2D::new(x, y))
    }

    /// `is_corner`
    pub fn is_corner(&self, x: UCoor2DIndex, y: UCoor2DIndex) -> bool {
        debug_assert!(self.check_index(x, y));
        is_corner(self.topology, self.width, self.height, &UCoor2D::new(x, y))
    }

    /// get reference to element on x, y
    pub fn get(&self, x: UCoor2DIndex, y: UCoor2DIndex) -> Option<&T> {
        if self.check_index(x, y) {
            Some(&self.data[self.index_to_vec_index(x, y)])
        } else {
            None
        }
    }

    /// get reference to element on x, y
    pub fn get_unchecked(&self, x: UCoor2DIndex, y: UCoor2DIndex) -> &T {
        &self.data[self.index_to_vec_index(x, y)]
    }

    /// get mutable reference element on x, y
    pub fn get_mut(&mut self, x: UCoor2DIndex, y: UCoor2DIndex) -> Option<&mut T> {
        if self.check_index(x, y) {
            let vec_index = self.index_to_vec_index(x, y);
            Some(&mut self.data[vec_index])
        } else {
            None
        }
    }

    /// set new element on x, y and return old element
    pub fn set(&mut self, x: UCoor2DIndex, y: UCoor2DIndex, new_value: T) -> T {
        debug_assert!(self.check_index(x, y));
        self.set_unchecked(x, y, new_value)
    }

    fn set_unchecked(&mut self, x: usize, y: usize, new_value: T) -> T {
        let vec_index = self.index_to_vec_index(x, y);
        replace(&mut self.data[vec_index], new_value)
    }

    /// set new element on x, y based on vector
    pub fn set_from_vec(&mut self, new_values: &[Vec<T>]) {
        debug_assert_eq!(new_values.len(), self.height);
        debug_assert_eq!(new_values[0].len(), self.width);
        for (y, row) in new_values.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                self.set_unchecked(x, y, cell.clone());
            }
        }
    }

    /// return all indexes
    pub fn all_indexes(&self) -> impl Iterator<Item = UCoor2D> {
        grid_iteration::all_cells(self.width, self.height)
    }

    /// return all neighbor indexes (based on topology and neighborhood)
    pub fn neighborhood_cell_indexes(
        &self,
        x: UCoor2DIndex,
        y: UCoor2DIndex,
    ) -> impl Iterator<Item = UCoor2D> {
        grid_iteration::neighborhood_cells(
            self.topology,
            self.width,
            self.height,
            UCoor2D::new(x, y),
            self.neighborhood,
        )
    }

    fn map_indexes_to_cells(
        &self,
        it: impl Iterator<Item = UCoor2D>,
    ) -> impl Iterator<Item = (UCoor2D, &T)> {
        it.map(|coor| (coor.clone(), self.get_unchecked(coor.x, coor.y)))
    }

    // map_indexes_to_cells_mut not possible to implement (multiple borrows of self_data)

    /// all data
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// return all elements
    pub fn all_cells(&self) -> impl Iterator<Item = (UCoor2D, &T)> {
        self.map_indexes_to_cells(self.all_indexes())
    }

    /// return all neighbor elements (based on topology and neighborhood)
    pub fn neighborhood_cells(
        &self,
        x: UCoor2DIndex,
        y: UCoor2DIndex,
    ) -> impl Iterator<Item = (UCoor2D, &T)> {
        self.map_indexes_to_cells(self.neighborhood_cell_indexes(x, y))
    }

    /// return adjacent cell in direction
    pub fn adjacent_cell(
        &self,
        x: UCoor2DIndex,
        y: UCoor2DIndex,
        direction: Direction,
    ) -> Option<UCoor2D> {
        adjacent_cell(
            self.topology,
            self.width,
            self.height,
            UCoor2D::new(x, y),
            direction,
        )
    }

    /// Print grid
    pub fn print(&self, add_stars: bool) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}{}",
                    self.get_unchecked(x, y),
                    if add_stars { "*" } else { "" }
                );
            }
            println!();
        }
    }

    /// Print grid (with extra newline)
    pub fn println(&self, add_stars: bool) {
        self.print(add_stars);
        println!();
    }

    fn swap(&mut self, x1: UCoor2DIndex, y1: UCoor2DIndex, x2: UCoor2DIndex, y2: UCoor2DIndex) {
        if (x1, y1) != (x2, y2) {
            let vec_index1 = self.index_to_vec_index(x1, y1);
            let vec_index2 = self.index_to_vec_index(x2, y2);
            self.data.swap(vec_index1, vec_index2);
        }
    }

    /// flip_horizontal
    pub fn flip_horizontal(&mut self) {
        for x in 0..self.width / 2 {
            for y in 0..self.height {
                self.swap(x, y, self.width - x - 1, y);
            }
        }
    }

    /// flip_vertical
    pub fn flip_vertical(&mut self) {
        for y in 0..self.height / 2 {
            for x in 0..self.width {
                self.swap(x, y, x, self.height - y - 1);
            }
        }
    }

    fn transform(&mut self, coors: impl Iterator<Item = UCoor2D>, swap_width_height: bool) {
        let new_data = coors
            .map(|coor| self.get_unchecked(coor.x, coor.y))
            .cloned()
            .collect_vec();
        if swap_width_height {
            swap(&mut self.width, &mut self.height);
        }
        self.data = new_data;
    }

    /// transpose
    pub fn transpose(&mut self) {
        self.transform(
            (0..self.width)
                .cartesian_product(0..self.height)
                .map(UCoor2D::from_tuple),
            true,
        );
    }

    /// rotate_cw
    pub fn rotate_cw(&mut self) {
        // rotate clockwise by 90°
        self.transform(
            (0..self.width)
                .cartesian_product((0..self.height).rev())
                .map(UCoor2D::from_tuple),
            true,
        );
    }

    /// rotate_ccw
    pub fn rotate_ccw(&mut self) {
        // rotate counter clockwise by 90°
        self.transform(
            ((0..self.width).rev())
                .cartesian_product(0..self.height)
                .map(UCoor2D::from_tuple),
            true,
        );
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    fn build_common_array() -> GridArrayBuilder<isize> {
        GridArrayBuilder::default().width(4).height(5).clone()
    }

    fn build_common_bounded_array() -> GridArrayBuilder<isize> {
        build_common_array().topology(Topology::Bounded).clone()
    }

    fn build_common_torus_array() -> GridArrayBuilder<isize> {
        build_common_array().topology(Topology::Torus).clone()
    }

    fn populate_with_enumerated(a: &mut GridArray<isize>) {
        for (i, coor) in a.all_indexes().enumerate() {
            a.set(coor.x, coor.y, i.try_into().unwrap());
        }
        debug_assert_eq!(a.get(0, 0), Some(&0));
    }

    fn standard_tests(a: &mut GridArray<isize>) {
        debug_assert_eq!(a.get(0, 0), Some(&0));
        debug_assert_eq!(a.get(10, 11), None);

        debug_assert_eq!(a.all_indexes().count(), a.width * a.height);
        debug_assert_eq!(a.all_indexes().dedup().count(), a.width * a.height);
        debug_assert_eq!(a.all_cells().count(), a.width * a.height);

        a.set(3, 2, -42);
        debug_assert_eq!(a.get(3, 2), Some(&-42));

        {
            let mut new_value = 42;
            swap(a.get_mut(2, 3).unwrap(), &mut new_value);
            debug_assert_eq!(a.get(2, 3), Some(&42));
            let (_, cell) = a
                .all_cells()
                .find(|(coor, _)| *coor == UCoor2D::new(2, 3))
                .unwrap();
            debug_assert_eq!(cell, &42);
        }

        /* See above not stable
        a.print();
        {
            let mut new_value = -11;
            let (_, _, cell) = a
                .all_cells_mut()
                .find(|(x, y, _)| (x, y) == (&2, &3))
                .unwrap();
            swap(cell, &mut new_value);

            let cell_value: isize = *cell;
            println!("{} , {} , {}", a.get(2, 3).unwrap(), cell_value, new_value);
            a.print();
            debug_assert_eq!(a.get(2, 3), Some(&-11));
        }
        */

        populate_with_enumerated(a);

        //a.print();
        a.flip_horizontal();
        debug_assert_eq!(a.get(0, 0), Some(&15));
        //println!();
        //a.print();
        a.flip_vertical();
        debug_assert_eq!(a.get(0, 0), Some(&19));
        //println!();

        a.transpose();
        debug_assert_eq!(a.get(0, 0), Some(&19));
        debug_assert_eq!(a.get(a.width - 1, 0), Some(&15));

        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.transpose();
        new_a.transpose();
        debug_assert_eq!(&new_a, a);

        check_rotate_cw(a);
        check_rotate_ccw(a);
    }

    fn check_rotate_cw(a: &mut GridArray<isize>) {
        populate_with_enumerated(a);
        a.rotate_cw();
        debug_assert_eq!(a.get(0, 0), Some(&3));
        debug_assert_eq!(a.get(a.width - 1, a.height - 1), Some(&16));
        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.rotate_cw();
        new_a.rotate_cw();
        new_a.rotate_cw();
        new_a.rotate_cw();
        debug_assert_eq!(&new_a, a);
    }

    fn check_rotate_ccw(a: &mut GridArray<isize>) {
        populate_with_enumerated(a);
        a.rotate_ccw();
        debug_assert_eq!(a.get(0, 0), Some(&15));
        debug_assert_eq!(a.get(a.width - 1, a.height - 1), Some(&4));
        populate_with_enumerated(a);
        let mut new_a = a.clone();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        debug_assert_eq!(&new_a, a);
        new_a.rotate_cw();
        new_a.rotate_ccw();
        new_a.rotate_ccw();
        new_a.rotate_cw();
        debug_assert_eq!(&new_a, a);
    }

    #[test]
    fn grid_bounded_square_array_tests() {
        let mut a: GridArray<isize> = build_common_bounded_array()
            .neighborhood(Neighborhood::Square)
            .build()
            .unwrap();
        standard_tests(&mut a);

        debug_assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 8);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                debug_assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 3);
            }
        }
    }

    #[test]
    fn grid_bounded_orthogonal_array_tests() {
        let mut a: GridArray<isize> = build_common_bounded_array()
            .neighborhood(Neighborhood::Orthogonal)
            .build()
            .unwrap();
        standard_tests(&mut a);
        debug_assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 4);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                debug_assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 2);
            }
        }
    }

    #[test]
    fn grid_torus_square_array_tests() {
        let mut a: GridArray<isize> = build_common_torus_array()
            .neighborhood(Neighborhood::Square)
            .build()
            .unwrap();
        standard_tests(&mut a);

        debug_assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 8);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                debug_assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 8);
            }
        }
    }

    #[test]
    fn grid_torus_orthogonal_array_tests() {
        let mut a: GridArray<isize> = build_common_torus_array()
            .neighborhood(Neighborhood::Orthogonal)
            .build()
            .unwrap();
        standard_tests(&mut a);

        debug_assert_eq!(a.neighborhood_cell_indexes(1, 1).count(), 4);
        for x in [0, a.width - 1] {
            for y in [0, a.height - 1] {
                debug_assert_eq!(a.neighborhood_cell_indexes(x, y).count(), 4);
            }
        }
    }
}

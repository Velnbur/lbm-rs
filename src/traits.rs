use crate::geometry;
use crate::grid;
use crate::io::vtk;
use crate::num;

/// TODO: move traits to appropriate modules

pub trait Distribution: Sized + Copy + Sync + Send {
    type Storage: AsRef<[num]> + AsMut<[num]> + Default;
    type AllIterator: Iterator<Item = Self>;

    fn all() -> Self::AllIterator;
    fn c_squ() -> num;
    fn direction(&self) -> geometry::Direction;
    fn from_direction(direction: geometry::Direction) -> Option<Self>;
    fn constant(&self) -> num;
    fn size() -> usize;
    fn value(&self) -> usize;
    fn center() -> Self;
    fn opposite(&self) -> Self;
}

pub type DistributionStorage<D> = <D as Distribution>::Storage;

pub trait DirectDistribution: Distribution {
    type DirectIterator: Iterator<Item = Self>;

    fn direct() -> Self::DirectIterator;
}

pub trait DiagonalDistribution: Distribution {
    type DiagonalIterator: Iterator<Item = Self>;

    fn diagonal() -> Self::DiagonalIterator;
}

pub trait Collision<D: Distribution>: Copy + Sync + Send {
    fn collision<H, IH>(&self, f_hlp: &H, idx_h: IH) -> D::Storage
    where
        IH: Fn(&H, D) -> num;
}

pub trait Physics: Copy + Sync + Send {
    type Distribution: Distribution;

    fn collision<FH, IFH>(
        &self,
        f_h: &FH,
        idx_f_h: IFH,
    ) -> DistributionStorage<Self::Distribution>
    where
        IFH: Fn(&FH, Self::Distribution) -> num;
    #[inline(always)]
    fn integral<F: Fn(Self::Distribution) -> num>(_: F) -> num {
        0.0
    }

    fn write<O, F>(
        &self,
        vtk_writer: vtk::CellDataWriter,
        _: O,
        _: F,
    ) -> vtk::CellDataWriter
    where
        F: Fn(grid::Idx, Self::Distribution) -> num,
        O: Fn(grid::Idx) -> bool,
    {
        vtk_writer
    }
}

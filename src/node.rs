const BRANCH_FACTOR: usize = 5;

#[derive(Clone)]
pub struct Root<T> {
    child: [Option<Box<Node<T>>>; BRANCH_FACTOR + 1],
    count: usize,
}

pub struct InternalNode {
    data: Vec<u8>
}

pub struct LeafNode<T> {

}

pub enum RRBVec {
    SatelliteData(usize)
    Root,
    Node,
    Parent,
}

impl<T> RRBVec<T> {
    pub fn new() -> RRBVec<T> {
        RRBVec {

        }
    }
    pub fn with_capacity(capacity: usize) -> RRBVec<T> {
        RRBVec {
            buf: 
        }
    }
}
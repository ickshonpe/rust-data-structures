
struct Quad {
    north_west: Box<Node>,
    north_east: Box<Node>,
    south_east: Box<Node>,
    south_west: Box<Node>
}

enum Node {
    White,
    Black,
    Split(Quad)
}

struct PointQuadTree {
    width: usize,
    root: Node
}


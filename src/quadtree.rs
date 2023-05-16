const N_BODIES: usize = 100;
const MAX_DEPTH: usize = 8;

pub type Position = (f32, f32);
type NodeID = usize;


pub struct Body {
    mass: f64,
    position: Position,
}

pub struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    fn intersects(&self, other: Rect) -> bool {
        !(other.x > self.x + self.w
            || other.x + other.w < self.x
            || other.y + other.h > self.y
            || other.y < self.y + self.h)
    }

    fn contains(&self, p: Position) -> bool {
        (self.x..=self.x + self.w).contains(&p.0) & (self.y..self.y + self.h).contains(&p.1)
    }

    fn split(&self) -> [Rect; 4] {
        // splits this rect into four smaller ones of equal size. we assume that the point a rect has is it's top left.
        [
            Rect::new(
                // top left
                self.x,
                self.y,
                self.w / 2.,
                self.h / 2.,
            ),
            Rect::new(
                // top right
                self.x + self.w / 2.,
                self.y,
                self.w / 2.,
                self.h / 2.,
            ),
            Rect::new(
                // bottom left
                self.x,
                self.y + self.h / 2.,
                self.w / 2.,
                self.h / 2.,
            ),
            Rect::new(
                // bottom right
                self.x + self.w / 2.,
                self.y + self.h / 2.,
                self.w / 2.,
                self.h / 2.,
            ),
        ]
    }
}


struct QuadTree {
    root: Node,
    elements: Vec<Body>
}

impl QuadTree {
    fn new(bounds: Rect) -> Self {
        QuadTree { root: Node::new(Some(1), bounds), elements: Vec::with_capacity(N_BODIES)}
    }

    fn insert(&mut self, element: Body) {
        self.elements.push(element);
        let id: NodeID = self.elements.len();
        let mut current = &mut self.root;
        loop {
            if current.element.is_none() {
                current.element = Some(id);
                return;
            }
            for (dir, rect) in current.rect.split().into_iter().enumerate() {
                if rect.contains(self.elements[id].position) {
                    if current.neighbors[dir].is_none() {
                        current.neighbors[dir] = Some(Box::new(Node::new(Some(id), rect)));
                    } else {
                        let current = current.neighbors[dir].as_mut().unwrap().as_mut();
                    }
                }
            }
        }
    }
}

pub struct Node {
    element: Option<NodeID>,
    rect: Rect,
    neighbors: Vec<Option<Box<Node>>>,
}

impl Node {
    fn new(element: Option<NodeID>, rect: Rect) -> Self {
        Node {
            element,
            rect,
            neighbors: vec![None, None, None, None],
        }
    }
}

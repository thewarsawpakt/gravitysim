const N_BODIES: usize = 100;
const MAX_DEPTH: usize = 8;

pub type Position = (f32, f32);

#[derive(Clone, Copy, Debug)]
pub struct Body {
    mass: f64,
    position: Position,
}

#[derive(Clone, Debug)]
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
                // obttom right
                self.x + self.w / 2.,
                self.y + self.h / 2.,
                self.w / 2.,
                self.h / 2.,
            ),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    element: Option<Body>,
    rect: Rect,
    neighbors: [Option<Box<Node>>; 4],
}

impl Node {
    fn new(element: Option<Body>, rect: Rect) -> Self {
        Node {
            element,
            rect,
            neighbors: [None, None, None, None],
        }
    }

    fn insert(current: &mut Node, body: Body) {
        if current.element.is_none() {
            current.element = Some(body);
            return;
        }
        for (dir, rect) in current.rect.split().into_iter().enumerate() {
            if rect.contains(body.position) {
                if current.neighbors[dir].is_none() {
                    current.neighbors[dir] = Some(Box::new(Node::new(Some(body), rect)));
                } else {
                    return Node::insert(current.neighbors[dir].as_mut().unwrap(), body);
                }
            }
        }
    }
}

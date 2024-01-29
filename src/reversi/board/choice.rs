use super::point::Point;

pub struct Choice {
    pub focus: Point,
    pub targets: Vec<Point>,
    pub count: usize,
}

impl Choice {
    pub fn new(focus: Point) -> Self {
        let targets: Vec<Point> = vec![];
        let count: usize = 0;

        Self {
            focus,
            targets,
            count,
        }
    }

    pub fn copy(&self) -> Choice {
        let focus = self.focus.copy();
        let count: usize = self.count;

        let mut result =  Choice::new(focus);

        for i in 0..self.targets.len() {
            result.targets.push(self.targets[i].copy());
        }

        result.count = count;

        return result;
    }
}

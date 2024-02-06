
#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
}


#[derive(PartialEq, Debug)]
pub struct Robot {
    dir: Direction,
    x: i32,
    y: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, dir: Direction) -> Self {
        return Robot { dir, x, y };
    }

    pub fn turn_right(mut self) -> Self {
        match self.dir {
            Direction::North => self.dir = Direction::East,
            Direction::South => self.dir = Direction::West,
            Direction::West => self.dir = Direction::North,
            Direction::East => self.dir = Direction::South,
        }
        self
    }

    pub fn turn_left(mut self) -> Self {
        self = Robot::turn_right(self);
        self = Robot::turn_right(self);
        self = Robot::turn_right(self);
        self
    }

    pub fn advance(mut self) -> Self {
        match self.dir {
            Direction::North => self.y += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
            Direction::East => self.x += 1,
        }
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Robot {
        for x in instructions.chars() {
            match x {
                'R' =>  {
                    self = self.turn_right();
                },
                'L' => {
                    self = self.turn_left();
                },
                'A' => {
                    self = self.advance();
                },
                _ => {}
            }
        };
        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}


#[test]
fn at_origin_facing_north() {
    let robot = Robot::new(0, 0, Direction::North);
    let robot_end = robot.turn_right();
    assert_eq!(robot_end.position(), (0, 0));
    assert_eq!(robot_end.direction(), &Direction::East);
}
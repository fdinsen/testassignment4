use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct Snake {
    body: LinkedList<Block>,
    game_size: (i32, i32),
    pub prev_dir: Direction,
}
#[derive(Debug, Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Collision {
    None,
    Apple,
    Snake,
}

impl Snake {
    //Construction
    pub fn new(
        x: i32,
        y: i32,
        size: i32,
        default_move_dir: Direction,
        game_size: (i32, i32),
    ) -> Self {
        let mut body = LinkedList::new();
        for i in 0..size {
            body.push_back(Block { x: x - i, y });
        }
        Snake {
            body,
            prev_dir: default_move_dir,
            game_size,
        }
    }
    pub fn init_snake(size: i32, default_move_dir: Direction, game_size: (i32, i32)) -> Snake {
        let x_loc = (game_size.0 / 2).abs();
        let y_loc = (game_size.1 / 2).abs();
        Self::new(x_loc, y_loc, size, default_move_dir, game_size)
    }

    //Snake Logic - public
    pub(crate) fn move_snake(&mut self, dir: &Direction) {
        match &dir {
            Direction::Up => self.perform_move_snake(0, -1),
            Direction::Down => self.perform_move_snake(0, 1),
            Direction::Left => self.perform_move_snake(-1, 0),
            Direction::Right => self.perform_move_snake(1, 0),
        }
    }
    pub(crate) fn check_collision(&self, apple_loc: (i32, i32)) -> Collision {
        let mut tmp = self.body.clone();
        tmp.pop_front();
        if Snake::intersects_body(&tmp, self.get_head_pos()) {
            return Collision::Snake;
        }
        if self.get_head_pos() == apple_loc {
            return Collision::Apple;
        }
        Collision::None
    }
    pub(crate) fn grow_snake(&mut self) {
        let (x, y) = self.get_tail_pos();
        self.body.push_back(Block { x, y });
    }

    //Snake logic - private
    fn perform_move_snake(&mut self, delta_x: i32, delta_y: i32) {
        let target =
            Snake::calculate_next_position(self.get_head_pos(), delta_x, delta_y, self.game_size);
        self.body.pop_back();
        self.body.push_front(target);
    }

    //Getters
    pub fn get_head_pos(&self) -> (i32, i32) {
        match self.body.front() {
            Some(head) => (head.x, head.y),
            None => panic!("Error: Snake has no head!"),
        }
    }
    pub fn get_tail_pos(&self) -> (i32, i32) {
        match self.body.back() {
            Some(tail) => (tail.x, tail.y),
            None => panic!("Error: Snake has no tail!"),
        }
    }
    pub fn get_length(&self) -> usize {
        self.body.len()
    }
    pub fn get_body(&self) -> &LinkedList<Block> {
        &self.body
    }

    //Static
    fn calculate_next_position(current_pos: (i32, i32),delta_x: i32,delta_y: i32,
                               game_size: (i32, i32),) -> Block {
        let (head_x, head_y) = current_pos;
        let target = Block {
            x: head_x + delta_x,
            y: head_y + delta_y,
        };
        if target.x >= game_size.0 {
            return Block {
                x: 0,
                y: head_y + delta_y,
            };
        }
        if target.x < 0 {
            return Block {
                x: game_size.0 - 1,
                y: head_y + delta_y,
            };
        }
        if target.y >= game_size.1 {
            return Block {
                x: head_x + delta_x,
                y: 0,
            };
        }
        if target.y < 0 {
            return Block {
                x: head_x + delta_x,
                y: game_size.0 - 1,
            };
        }
        target
    }

    pub(crate) fn intersects_body(body: &LinkedList<Block>, other: (i32, i32)) -> bool {
        for body_part in body {
            if *body_part == other {
                return true;
            }
        }
        return false;
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Block) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialEq<(i32, i32)> for Block {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}
impl Direction {
    pub(crate) fn opposite(&self) -> Direction {
        match self {
            Self::Up => Direction::Down,
            Self::Down => Direction::Up,
            Self::Left => Direction::Right,
            Self::Right => Direction::Left,
        }
    }
}

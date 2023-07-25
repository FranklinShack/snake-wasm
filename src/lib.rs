mod random;
use crate::random::random_range;
use std::collections::VecDeque;


pub type Position = (usize, usize);

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    game_over: bool,
    points: usize,
}

impl SnakeGame {
    pub fn new (width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            snake: [((width / 2).max(0), (height / 2).max(0))].into_iter().collect(),
            direction: Direction::Up,
            food: (2.min(width-1), 2.min(height-1)),
            game_over: false,
            points: 0,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up) |
            (Direction::Up, Direction::Down) |
            (Direction::Right, Direction::Right) |
            (Direction::Right, Direction::Left) |
            (Direction::Down, Direction::Up) |
            (Direction::Down, Direction::Down) |
            (Direction::Left, Direction::Down) |
            (Direction::Left, Direction::Left) => {},
            (_, direction) => self.direction = direction,
        }
    }

    pub fn in_bounds(&self, (x, y): Position) -> bool {
        (x < self.width && y < self.height) && (x > 0 && y > 0)
    }

    pub fn tick(&mut self) {
        //Move snake

        if self.game_over  || self.snake.len() == 0{
            return;
        }
        
        let (x, y) = self.snake[0];

        let new_head = match &self.direction {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        };
        

        if !self.in_bounds(new_head) || self.snake.contains(&new_head){
            self.game_over = true;
        } else {
            if new_head != self.food {
                self.snake.pop_back();
            } else {
                let free_positions = (0..self.height)
                    .flat_map(|y| (0..self.width).map(move |x| (x, y)))
                    .filter(|pos| !self.snake.contains(pos))
                    .collect::<Vec<_>>();
            
                if free_positions.is_empty() {
                    self.game_over = true;
                    return;
                }

                self.food = free_positions[random_range(0, free_positions.len())];
            }
            self.snake.push_front(new_head);
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("{:?}", SnakeGame::new(10, 10));
    }
}
use serde::Serialize;
use std::collections::LinkedList;

use crate::gamestate::physics::{Direction, Position};

#[derive(Debug, Serialize)]
pub struct Snake {
    direction: Direction,
    head: Position,
    tail: LinkedList<Position>,
    #[serde(skip)]
    updated_tail_pos: bool,
    initial_length: u32,
}

impl Snake {
    pub fn new(head: Position, snake_length: u32) -> Self {
        let (x, y) = (head.x, head.y);
        let mut tail = LinkedList::new();

        for i in 1..(snake_length + 1) {
            tail.push_back(Position { x, y: y - i as i32 });
        }

        Self {
            direction: Direction::Down,
            head: Position { x, y },
            tail,
            updated_tail_pos: false,
            initial_length: snake_length,
        }
    }

    pub fn update(&mut self, height: u32, width: u32) {
        if self.tail.len() > 0 {
            self.tail.push_front(self.head.clone());
            self.tail.pop_back();
        }

        match self.direction {
            Direction::Up => self.head.y -= 1,
            Direction::Right => self.head.x += 1,
            Direction::Down => self.head.y += 1,
            Direction::Left => self.head.x -= 1,
        }

        if self.head.x >= height as i32 {
            self.head.x = 0;
        } else if self.head.y >= width as i32 {
            self.head.y = 0;
        } else if self.head.y < 0 {
            self.head.y = width as i32;
        } else if self.head.x < 0 {
            self.head.x = height as i32;
        }

        self.updated_tail_pos = true;
    }

    pub fn set_dir(&mut self, dir: Direction) {
        if dir == self.direction.opposite() || !self.updated_tail_pos {
            return;
        }

        self.direction = dir;
        self.updated_tail_pos = false;
    }

    pub fn get_head_pos(&self) -> &Position {
        &self.head
    }

    pub fn get_len(&self) -> usize {
        &self.tail.len() - self.initial_length as usize
    }

    pub fn is_tail_overlapping(&self) -> bool {
        for pos in self.tail.iter() {
            if *pos == self.head {
                return true;
            }
        }

        false
    }

    pub fn will_tail_overlapp(&self) -> bool {
        let next = self.next_head_pos();

        for pos in self.tail.iter() {
            if *pos == next {
                return true;
            }
        }

        false
    }

    pub fn grow(&mut self) {
        let last = match self.tail.back() {
            Some(pos) => pos.clone(),
            None => self.head.clone(),
        };

        self.tail.push_back(last);
    }

    fn next_head_pos(&self) -> Position {
        let mut pos = self.head.clone();

        match self.direction {
            Direction::Up => pos.y -= 1,
            Direction::Left => pos.x -= 1,
            Direction::Down => pos.y += 1,
            Direction::Right => pos.x += 1,
        }

        pos
    }
}

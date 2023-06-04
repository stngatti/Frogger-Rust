use core::num;
use std::any::Any;
use std::cmp::{min, max};

use crate::{actor::*, log, rand, g2d};
use crate::rand::*;
use crate::g2d::*;

pub struct Raft {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
}
impl Raft {
    pub fn new(pos: Pt, speed: i32) -> Raft {
        Raft{pos: pos, step: pt(1, 0), size: pt(96 , 32), speed: speed}
    }
    pub fn get_speed(&self) -> i32 { self.speed }
}
impl Actor for Raft {
    fn act(&mut self, arena: &mut ArenaStatus) {

        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }

        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        Some(pt(192 , 96))
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Crocodile {
    pos: Pt,
    size: Pt,
    step: Pt,
    speed: i32,
}
impl Crocodile {
    pub fn new(pos: Pt, speed: i32) -> Crocodile {
        Crocodile{pos: pos, step: pt(1, 0), size: pt(96 , 32), speed: speed}
    }
    pub fn get_speed(&self) -> i32 { self.speed }
}
impl Actor for Crocodile {
    fn act(&mut self, arena: &mut ArenaStatus) {

        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }

        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        Some(pt(192, 224)) 
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Turtle {
    pos: Pt,
    step: Pt,
    size: Pt,
    under_water: bool,
    new_sprite: i32,
    sprite_fps: i32,
    sprite_tick: i32,
    speed: i32,
}
impl Turtle {
    pub fn new(pos: Pt, speed: i32, under_water: bool, sprite_tick: i32) -> Turtle {
        Turtle{pos: pos, step: pt(1, 0), size: pt(32 , 32), speed: speed, under_water: under_water, new_sprite: 0, sprite_fps: 20, sprite_tick: sprite_tick}
    }
    pub fn get_speed(&self) -> i32 { self.speed }
}
impl Actor for Turtle {
    fn act(&mut self, arena: &mut ArenaStatus) {
        self.new_sprite += 1;
        if self.under_water  && self.new_sprite > (self.sprite_tick + self.sprite_fps * 6) {
            self.new_sprite = self.sprite_tick; 
        }
        else if !self.under_water && self.new_sprite >= (self.sprite_tick + self.sprite_fps * 3) {
            self.new_sprite = self.sprite_tick;
        }   

        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }

        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        if self.new_sprite >= self.sprite_tick && self.new_sprite < (self.sprite_fps + self.sprite_tick) { Some(pt(256, 128)) }
        else if self.new_sprite >= (self.sprite_fps + self.sprite_tick) && self.new_sprite < (self.sprite_fps * 2 + self.sprite_tick) { Some(pt(224, 128)) }
        else if self.new_sprite >= (self.sprite_fps * 2 + self.sprite_tick) && self.new_sprite < (self.sprite_fps * 3 +  self.sprite_tick) { Some(pt(192, 128)) }
        else if self.new_sprite >= (self.sprite_fps * 3 + self.sprite_tick) && self.new_sprite < (self.sprite_fps * 4 + self.sprite_tick) { Some(pt(192, 160)) }
        else if self.new_sprite >= (self.sprite_fps * 4 + self.sprite_tick) && self.new_sprite < (self.sprite_fps * 5 + self.sprite_tick) { Some(pt(224, 160)) }
        else { None }
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Vehicle {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    car: bool,
    car_type: i32,
}
impl Vehicle {
    pub fn new(pos: Pt, car: bool, speed: i32) -> Vehicle {
        Vehicle{pos: pos, step: pt(1, 0), size: pt(if car { 32 } else { 64 }, 32), speed: speed, car:car, car_type: randint(6, 9)}
    }
}
impl Actor for Vehicle {
    fn act(&mut self, arena: &mut ArenaStatus) {
 
        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }

        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        if self.car {
            Some(pt(self.car_type * 32, if self.speed > 0 && self.car_type == 8 { 32 } 
                else if self.speed < 0 && self.car_type == 8 { 0 } 
                else if self.speed > 0 { 0 } else { 32 } )) 
        }
        else
        {
            Some(pt(if self.speed >= 0 { 256 } else { 192 }, 64))
        }
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Water {
    pos: Pt,
    size: Pt
}
impl Water {
    pub fn new(pos: Pt) -> Water {
        Water{pos: pos, size: pt(640, 192)}
    }
}
impl Actor for Water {
    fn act(&mut self, _arena: &mut ArenaStatus) {
    }
    fn sprite(&self) -> Option<Pt> { None }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Frog {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    lives: i32,
    count_steps: i32,
    drag: i32,
    on_raft: bool,
    on_water: bool,
    direction: String, 
}
impl Frog {
    pub fn new(pos: Pt) -> Frog {
        Frog{pos: pos, step: pt(0, 0), size: pt(32, 32),
            speed: 32, lives: 5, count_steps: 0, drag: 0, on_raft: false, on_water: false, direction: "Up".to_string(),}
    }
    fn lives(&self) -> i32 { self.lives }
}
impl Actor for Frog {
    fn act(&mut self, arena: &mut ArenaStatus) {

            self.on_raft = false;
            self.on_water = false;

            for other  in arena.collisions() {
                if let Some(_) = other.as_any().downcast_ref::<Vehicle>() {
                    self.lives -= 1;
                    self.pos = pt(arena.size().x/2, arena.size().y - 32);
                    self.direction = "Up".to_string();
                }
                if let Some(raft) = other.as_any().downcast_ref::<Raft>() {
                    self.on_raft = true;
                    if self.count_steps == 0 { self.drag = raft.get_speed(); }
                }
                if let Some(_) = other.as_any().downcast_ref::<Water>() { self.on_water = true; }
                if let Some(turtle) = other.as_any().downcast_ref::<Turtle>() {
                    self.on_raft = true;
                    if self.count_steps == 0 { self.drag = turtle.get_speed(); }
                    if !other.sprite().is_some() { self.on_raft = false; }
                }
                if let Some(crocodile) = other.as_any().downcast_ref::<Crocodile>() {
                    self.on_raft = true;
                    if self.count_steps == 0 { self.drag = crocodile.get_speed(); }
                }
            }

        if !(self.on_raft || !self.on_water)  {
            self.lives -= 1;
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
            self.direction = "Up".to_string();
        }
        else if self.on_water && !self.on_raft {
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
        }
        else if self.on_water { }

        let keys = arena.current_keys();
        self.step = pt(0, 0);

        if self.count_steps == 0 {

            if keys.contains(&"ArrowUp") {
                self.count_steps = self.speed;
                self.step.x = 0;
                self.step.y = -self.speed;
                self.direction = "Up".to_string();
            } 
            if keys.contains(&"ArrowDown") {
                self.count_steps = self.speed;
                self.step.x = 0;
                self.step.y = self.speed;
                self.direction = "Down".to_string();
            }
            if keys.contains(&"ArrowLeft") {
                self.count_steps = self.speed;
                self.step.x = -self.speed;
                self.step.y = 0;
                self.direction = "Left".to_string();
            } 
            if keys.contains(&"ArrowRight") {
                self.count_steps = self.speed;
                self.step.x = self.speed;
                self.step.y = 0;
                self.direction = "Right".to_string();
            }
        }
        if self.count_steps > 0 {
            self.pos.x += self.step.x;
            self.pos.y += self.step.y;
            self.count_steps -= 16;
        }
        self.pos.x += self.drag;
        self.drag = 0;

        let scr = arena.size() - self.size;
        if ! self.on_water {
            self.pos.x = min(max(self.pos.x, 0), scr.x); 
            self.pos.y = min(max(self.pos.y, 0), scr.y);  
        }

        if self.pos.x > 640 || self.pos.x < -32 {
            self.lives -= 1;
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
            self.direction = "Up".to_string();
        }

    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> {
        if self.direction == "Up" { Some(pt(0 + 32*(self.pos.y % 3), 0)) }
        else if self.direction == "Down" { Some(pt(160 - 32*(self.pos.y % 3), 32)) }
        else if self.direction == "Left" { 
            if self.drag  == 0 {
            Some(pt(96 + 32*(self.pos.x % 3), 0))
            } else { Some(pt(96, 0))}
        }
        else if self.direction == "Right" { 
            if self.drag  == 0 {
                Some(pt(64 - 32*(self.pos.x % 3), 32)) 
            } else { Some(pt(64, 32))}
        }
        else { None } 
        }
    fn alive(&self) -> bool { self.lives > 0 }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct BounceGame {
    arena: Arena,
}
impl BounceGame {
    pub fn new(size: Pt, nvehicles: i32, nrafts: i32, nturtles: i32) -> BounceGame {
        let mut arena: Arena = Arena::new(size);
        arena.spawn(Box::new(Water::new(pt(0,32))));

        for i in 0..5 {
            let mut position = 0;
            let mut speed = randint(1, 5);
            
            if  i%2 != 0 { speed = - speed }
            for _ in 0..nvehicles {
                let car = randint(0, 1);
                arena.spawn(Box::new(Vehicle::new(pt(position, 384-(32*i)), if car == 1 {true} else {false}, speed)));
                position += randint(70, 250);
            }
        }

        for i in 0..6 {
            let mut position = 0;
            let speed = randint(1, 3);


            if i%2 == 0 {
                for _ in 0..nturtles {

                    let sprite_tick = randint(1, 10);
                    let under_water = if sprite_tick % 2 == 0 { true } else { false };
                    for n in 0..randint(2, 3)
                    {
                        arena.spawn(Box::new(Turtle::new(pt(position + 32*n, 192-(32*i)), -speed, under_water, sprite_tick)));
                    }
                    position += randint(100, 250); 
                }
            }
            else {
                for _ in 0..nrafts {
                    let choice = randint(0,1);
                    if choice == 0 {
                        arena.spawn(Box::new(Crocodile::new(pt(position, 192-(32*i)), speed)));
                    }
                    else {
                        arena.spawn(Box::new(Raft::new(pt(position, 192-(32*i)), speed)));
                    }
                    position += randint(100, 250);
                }
            }
            arena.spawn(Box::new(Frog::new(pt(arena.size().x/2, arena.size().y - 32))));
        }
        BounceGame{arena: arena}
    }
    pub fn game_over(&self) -> bool { self.remaining_lives() == 0 }
    pub fn game_won(&self) -> bool { 
        let mut position: i32 = 0;
        let actors = self.actors();
        for a in actors {
            if let Some(frog) = a.as_any().downcast_ref::<Frog>() { position = frog.pos().y; }
        }
        if position  == 0 { true } else { false }
    }
    pub fn remaining_lives(&self) -> i32 {
        let mut lives: i32 = 0;
        let actors = self.actors();
        for a in actors {
            if let Some(frog) = a.as_any().downcast_ref::<Frog>() { 
                lives = frog.lives();
                return lives; 
            }
        }
        lives
    }
    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
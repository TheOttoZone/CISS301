use::rust_fsm::*;
use::std::io;

state_machine!{
    #[state_machine(input(crate::CharInput), state(crate::CharStates))]
    pub character_machine(Standing)

    Standing(Walk) => Walking,
    Standing(Walk) => Walking,
    Standing(Jump) => Jumping,
    Standing(Lay) => Laying,
    Walking(Stop) => Standing,
    Walking(Jump) => Jumping,
    Walking(Walk) => Running,
    Walking(Lay) => Laying,
    Running(Stop) => Standing,
    Running(Jump) => Jumping,
    Running(Lay) => Dead,
    Jumping(Fall) => Falling,
    Falling(Stop) => Standing,
    Laying(Walk) => Walking,
    Laying(Stop) => Standing,
    Laying(Jump) => Standing,
}

pub enum CharInput{
    Walk,
    Stop,
    Jump,
    Lay,
    Fall
}

#[derive(PartialEq, Debug)]
pub enum CharStates{
    Standing,
    Walking,
    Running,
    Jumping,
    Laying,
    Falling,
    Dead,
}

#[derive(PartialEq, Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}


struct Character{
    machine: character_machine::StateMachine,
    direction: Direction,
    jump_counter: i32,
    x: i32,
    y: i32,
}

impl Character {
    pub fn new() -> Self {
        Self {
            machine: StateMachine::new(),
            direction: Direction::Right,
            jump_counter: 0,
            x: 0,
            y: 0,
        }
    }
    pub fn input(&mut self, input_direction:Direction){
        match input_direction{
            Direction::Up =>{
                if self.machine.state() == &CharStates::Jumping{
                    println!("You can't double jump!")
                }else{
                    let _ = self.machine.consume(&CharInput::Jump);
                }
            }
            Direction::Down =>{
                let _ = self.machine.consume(&CharInput::Lay);
            }
            Direction::Left =>{
                if self.direction == Direction::Right{
                    let _ = self.machine.consume(&CharInput::Stop);
                    self.direction = Direction::Left;
                }else{
                    self.direction = Direction::Left;
                    let _ = self.machine.consume(&CharInput::Walk);
                }

            },
            Direction::Right =>{
                if self.direction == Direction::Left{
                    let _ = self.machine.consume(&CharInput::Stop);
                    self.direction = Direction::Right
                }else{
                    self.direction = Direction::Right;
                    let _ = self.machine.consume(&CharInput::Walk);
                }
            },
            
        }
    }
    pub fn update(&mut self){
        if let CharStates::Walking = self.machine.state(){
            match self.direction{
                Direction::Up => {}, // unused
                Direction::Down => {}, // unsused
                Direction::Left => {
                    self.x -= 1
                },
                Direction::Right => {
                    self.x += 1
                },

            }
        }
        if let CharStates::Running = self.machine.state(){
            match self.direction{
                Direction::Up => {}, // unused
                Direction::Down => {}, // unsused
                Direction::Left => {
                    self.x -= 2
                },
                Direction::Right => {
                    self.x += 2
                },

            }
        }
        if let CharStates::Jumping = self.machine.state(){ 
            if self.jump_counter < 3{
                self.y += 1;
                self.jump_counter += 1
            }else{
                let _ = self.machine.consume(&CharInput::Fall);
                self.jump_counter = 0;
            }
        }
        if let CharStates::Falling = self.machine.state(){
            if self.y > 0{
                self.y -= 1
            }else{
                let _ = self.machine.consume(&CharInput::Stop);
            }
        }
        if let CharStates::Laying = self.machine.state(){
            if self.direction == Direction::Down{
                println!("You can't lay down harder!");
            }
        }
    }
}

fn main() {
    println!("Welcome to the totally real game!");
    let mut character = Character::new();

    while character.machine.state() != &CharStates::Dead{
        println!("Your character is {:?} {:?}. Their position is ({}, {}).", character.machine.state(), character.direction, character.x, character.y);

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // reduce input line to the first character to avoid having to deal with variable length input
        if input.trim().len() > 1{
            input.pop();
        }

        match input.trim().to_lowercase().as_str(){
            "w" => {
                character.input(Direction::Up);
            }
            "s" => {
                character.input(Direction::Down);
            }
            "a" => {
                character.input(Direction::Left);
            }
            "d" => {
                character.input(Direction::Right);
            }
            "" => {}
            _ => {
                println!("Invalid input!");
            }
        }
        character.update();
    }
}

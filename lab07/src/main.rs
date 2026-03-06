use::rust_fsm::*;
use::std::io;

state_machine!{
    #[state_machine(input(crate::CharInput), state(crate::CharStates))]
    pub character_machine(Standing)

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
    pub fn new() -> Self { // character contsrtructor
        Self {
            machine: StateMachine::new(),
            direction: Direction::Right,
            jump_counter: 0, // tracks how long the character has been in the air
            x: 0,
            y: 0,
        }
    }
    pub fn input(&mut self, input_direction:Direction){ // handle most state transitions from user input
        match input_direction{
            Direction::Up =>{
                if self.machine.state() == &CharStates::Jumping{ // jumping in air doesnt make sense
                    println!("You can't double jump!")
                }else{
                    let _ = self.machine.consume(&CharInput::Jump);
                }
            }
            Direction::Down =>{
                let _ = self.machine.consume(&CharInput::Lay);
            }
            Direction::Left =>{
                if self.direction == Direction::Right{ // handle turning right
                    let _ = self.machine.consume(&CharInput::Stop);
                    self.direction = Direction::Left;
                }else{
                    let _ = self.machine.consume(&CharInput::Walk);
                }

            },
            Direction::Right =>{
                if self.direction == Direction::Left{ // handle turning left
                    let _ = self.machine.consume(&CharInput::Stop);
                    self.direction = Direction::Right
                }else{
                    let _ = self.machine.consume(&CharInput::Walk);
                }
            },
            
        }
    }
    pub fn update(&mut self){ // game update logic
        if let CharStates::Walking = self.machine.state(){
            match self.direction{
                Direction::Left => {
                    self.x -= 1
                },
                Direction::Right => {
                    self.x += 1
                },
                _ => {}
            }
        }
        if let CharStates::Running = self.machine.state(){
            match self.direction{
                Direction::Left => {
                    self.x -= 2
                },
                Direction::Right => {
                    self.x += 2
                },
                _ => {}
            }
        }
        if let CharStates::Jumping = self.machine.state(){ 
            if self.jump_counter < 3{
                self.y += 1;
                self.jump_counter += 1;
            }else{
                let _ = self.machine.consume(&CharInput::Fall);
                self.jump_counter = 0;
            }
        }
        if let CharStates::Falling = self.machine.state(){
            if self.y > 0{
                self.y -= 1;
                if self.y == 0{
                    let _ = self.machine.consume(&CharInput::Stop);
                }
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
    println!("Welcome to the totally real game! Use W, A, S, D to move and press enter to send your input. Enter can also be used to simply continue.\n");
    let mut character = Character::new();

    while character.machine.state() != &CharStates::Dead{
        println!("Your character is {:?} {:?}. Their position is ({}, {}).", character.machine.state(), character.direction, character.x, character.y);

        // read input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // reduce input line to the first character to avoid having to deal with variable length input
        while input.trim().len() > 1{
            input.pop();
        }

        // handle input
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
    println!("Your character tripped really really hard! Your charcter is dead.")
}

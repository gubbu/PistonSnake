extern crate piston_window;
mod gamewindow;
//mod fontrender;
const HEIGHT: u32 = 14;
//the blocks the maps consists of. beware! To simplify code this map is mirrored horizontal in the actual game read if from right to left.
const MAP: [u32; HEIGHT as usize] = [
    0b11111111111111111111111111111111,
    0b10000000000000100000000000000001,
    0b10000000000000100000000000000001,
    0b10000000000000100000000000000001,
    0b10000000000000000000000000000001,
    0b11110000000000000000000000000001,
    0b10000000000000000000000000000001,
    0b10000000000000100000000011111111,
    0b10000000000000100000000001000001,
    0b10000000000000100000000000000001,
    0b10000001111111100000000000000001,
    0b10000000000000100000000001000001,
    0b10000000000000100000000001000001,
    0b11111111111111111111111111111111
];

const GAMESIZE: (u32, u32) = (32, HEIGHT);
const TILESIZE: f64 = 15.0;
const SNAKECOLOR: [f32;4] = [0.0, 0.5, 0.0, 0.5];
const FRUITCOLOR: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const BG: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const BLOCKCOLOR: [f32;4] = [0.0, 0.0, 1.0, 1.0];
const SECONDS_PER_UPDATE: f64 = 0.25;



//simple demo of the two modules gamewindow and fontrender
struct Snakegame{
    fruitpos: (i32, i32),
    snake: Vec<(i32, i32)>,
    currentdirection: (i32, i32),
    //handling 
    timer: f64,
    //as the snake player hets larger, the seconds per update will decrease towards 0.25 seconds
    seconds_per_update: f64,
    gameover: bool,
}

fn random(seed: f32)->f32{
    //very dump random number generator generates numbers between 0.0 and 1.0
    return ((seed*398906.234525).cos()*3449.32499).fract().abs();
}

impl Snakegame{

    //moves the snkae based on self.currentdirection, returns true if GAMEOVER
    fn snakemove(&mut self)->bool{
        if let Some(&position) = self.snake.last(){
            let mut gameover = false;
            if position.0+self.currentdirection.0 == self.fruitpos.0 && position.1+self.currentdirection.1 == self.fruitpos.1{
                self.snake.push(self.fruitpos);
                self.nextfruitpos();
                println!("have eaten fruit, next fruit at {:?}", self.fruitpos);
            }else{
                //move the snake: position gets depricated:
                self.snake.push((position.0+self.currentdirection.0, position.1+self.currentdirection.1));
                self.snake.remove(0);
                //only if there is no collsion with a fruit there willbe wall/MAP collison checks:
                //bound conditions:
                if position.0 < 0 || position.0 >= 31 ||position.1 < 0 || position.1 >= MAP.len() as i32{
                    gameover = true;
                    println!("OUT OF BOUNDS");
                }else{
                    gameover = MAP[position.1 as usize]&(1<<position.0) != 0;
                    if gameover{println!("WALL COLLISION");}
                } 
            }
            if !gameover{
                //check if the snake collides with itself:
                for (i, snakeelement) in self.snake.iter().enumerate(){
                    if i!=self.snake.len()-1{
                        if snakeelement == &self.snake[self.snake.len()-1]{
                            println!("SELF COLLISION: index {} collides with head {:?}", i, self.snake);
                            gameover = true;
                            break;
                        }
                    }
                }
            }
            println!("moving current pos{:?}, gameover {}", position, gameover);
            return gameover;
        }
        return true;
    }

    fn nextfruitpos(&mut self){
        //generate new fruit at "pseudorandom" position
        self.fruitpos.0 = (random(self.snake.len() as f32+self.fruitpos.1 as f32/3.0)*(GAMESIZE.0-2) as f32) as i32+1;
        self.fruitpos.1 = (random(self.snake.len() as f32+self.fruitpos.0 as f32/3.0)*(GAMESIZE.1-2) as f32) as i32+1;
        //make the game faster everytime the layer picks up a fruit. SECONDS_PER_UODATE seconds per update is minmum (=4 fps)
        self.seconds_per_update = SECONDS_PER_UPDATE+1.0/self.snake.len() as f64;
    }

    fn newstandard()->Self{
        Snakegame{fruitpos: (5, 9),
        snake: vec![(1,1), (1, 2)],
        currentdirection:(0,1),
        timer:0.0,
        seconds_per_update: SECONDS_PER_UPDATE+0.5,
        gameover: false}
    }

    fn start(){
        gamewindow::makegame("SNAKE", [GAMESIZE.0*TILESIZE as u32, GAMESIZE.1*TILESIZE as u32,], 30, 30, &mut Snakegame::newstandard());
    }
}

//const MESSAGE: String = String::from("HELLO WORLD!+-*/ WORLD: HOW ARE YOU\nI AM NOT OK.");
//implementing the gametrait for Gametest.
impl gamewindow::Gametrait for Snakegame{
    fn onstart(&mut self){println!("starting")}

    fn update(&mut self, dt: f64){
        //println!("delta time {}", dt)
        self.timer += dt;
        if self.timer > SECONDS_PER_UPDATE{
            self.timer = 0.0;
            if self.snakemove(){
                //println!("GAMEOVER");
                self.gameover = true;
            }
        }
    }

    fn render(&self, g: &mut piston_window::G2d, transform: [[f64; 3]; 2]){
        //lets render the map!
        for (y, row) in MAP.iter().enumerate(){
            for tile in 0..32{
                if row&(1<<tile) != 0{
                    piston_window::rectangle(BLOCKCOLOR, [tile as f64*TILESIZE, y as f64*TILESIZE, TILESIZE, TILESIZE], transform, g);
                }
            }
        }
        piston_window::clear(BG, g);
        piston_window::rectangle(FRUITCOLOR, [TILESIZE*self.fruitpos.0 as f64, TILESIZE*self.fruitpos.1 as f64, TILESIZE, TILESIZE], transform, g);
        for snakepart in self.snake.iter(){
            piston_window::rectangle(SNAKECOLOR, [TILESIZE*snakepart.0 as f64, TILESIZE*snakepart.1 as f64, TILESIZE, TILESIZE], transform, g);
        }

    }
    
    fn shouldquit(&self)->bool{return self.gameover;}
    fn onquit(&mut self){println!("QUITING!");}
    fn keyboard(&mut self, ispressed: bool, keychar: char){
        if ispressed{
            if keychar == 'W'{self.currentdirection = (0, -1);}
            else if keychar == 'A'{self.currentdirection = (-1, 0);}
            else if keychar == 'S'{self.currentdirection = (0, 1);}
            else if keychar == 'D'{self.currentdirection = (1, 0);}
        }
    }    
}

fn main() {
    println!("snakegame controls wasd.");
    Snakegame::start();
}
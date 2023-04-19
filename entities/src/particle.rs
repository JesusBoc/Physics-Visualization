use std::f64::consts::PI;

use sdl2::rect::Point;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Rect;

use crate::vector::Vector;

pub const TIME_STEP: f64 = 0.001;
const VEL_COLOR: Color = Color::BLUE;
const ACC_COLOR: Color = Color::RED;
const MAX_SAVED_POINTS: usize = 100;

pub struct ParticleBuilder{
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
    size: u32,
    graph_position: bool,
    height: u32,
    width: u32,
}

impl ParticleBuilder{
    pub fn new(height: u32, width: u32) -> Self {
        let builder = ParticleBuilder {
            x: 0.0,
            y: (height / 2) as f64,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            size: 14,
            graph_position: false,
            height,
            width,
        };
        builder
    }
    pub fn add_vx(mut self, vx: f64) -> Self{
        self.vx += vx;
        self
    }
    pub fn set_vx(mut self, vx: f64) -> Self{
        self.vx = vx;
        self
    }
    pub fn add_vy(mut self, vy: f64) -> Self{
        self.vy += vy;
        self
    }
    pub fn set_vy(mut self, vy: f64) -> Self{
        self.vy = vy;
        self
    }
    pub fn set_pos(mut self, x: f64, y: f64) -> Self{
        self.x = x;
        self.y = y;
        self
    }
    pub fn add_ax(mut self, ax: f64) -> Self{
        self.ax += ax;
        self
    }
    pub fn set_ax(mut self, ax: f64) -> Self{
        self.ax = ax;
        self
    }
    pub fn add_ay(mut self, ay: f64) -> Self{
        self.ay += ay;
        self
    }
    pub fn set_ay(mut self, ay: f64) -> Self{
        self.ay = ay;
        self
    }
    pub fn track_position(mut self, v: bool) -> Self{
        self.graph_position = v;
        self
    }
    pub fn build(self) -> Particle{
        let built = Particle::new(
            self.x,
            self.y,
            self.vx,
            self.vy,
            self.ax,
            self.ay,
            self.size,
            self.graph_position,
            self.width,
            self.height,
        );
        built
    }
}

#[derive(Clone, Copy)]
pub struct Particle{
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
    pos: Point,
    vel: Vector,
    acc: Vector,
    size: u32,
    position_reg: [Point; MAX_SAVED_POINTS],
    saved_points: usize,
    graph_position: bool,
    scene_width: u32,
    scene_height: u32,
}

impl Particle{
    pub fn new(x: f64,y: f64,vx: f64,vy: f64,ax: f64,ay: f64,size: u32, graph_position: bool, scene_width: u32, scene_height: u32) -> Self {
        let pos = Point::new(x as i32, y as i32);
        let vel = Vector::new(x as i32, y as i32, (x + vx/4.0) as i32, (y + vy/4.0) as i32, VEL_COLOR);
        let acc = Vector::new(x as i32, y as i32, (x + ax/4.0) as i32, (y + ay/4.0) as i32, ACC_COLOR);
        let position_reg = [Point::new(0, 0);MAX_SAVED_POINTS];
        let saved_points = 0;
        let new_particle = Particle{x,y,vx,vy,ax,ay,pos,size,vel,acc,position_reg, graph_position,scene_height,scene_width, saved_points};
        new_particle
    }
    pub fn update(&mut self){
        self.x += self.vx*TIME_STEP;
        self.y += self.vy*TIME_STEP;
        if self.y > (self.scene_height - self.size/2) as f64{
            self.vy = -self.vy*0.9;
            self.y = (self.scene_height - self.size/2) as f64;
        } 
        if self.y <= 0.0{
            self.vy = -self.vy*0.9;
            self.y = 0.0;
        }
        if self.x > (self.scene_width - self.size/2) as f64{
            self.vx = -self.vx*0.9;
            self.x = (self.scene_width - self.size/2) as f64;
        }
        if self.x < 0.0{
            self.vx = -self.vx*0.9;
            self.x = 0.0;
        }
        self.vx += self.ax*TIME_STEP;
        self.vy += self.ay*TIME_STEP;
        let x = self.x as i32;
        let y = self.y as i32;
        let vx = (self.vx/4.0) as i32;
        let vy = (self.vy/4.0) as i32;
        let ax = (self.ax/4.0) as i32;
        let ay = (self.ay/4.0) as i32;
        self.pos = Point::new(x, y);
        self.ax = 0.0;
        self.ay = 0.0;
        self.vel = Vector::new(x, y, x+vx, y+vy as i32, VEL_COLOR);
        self.acc = Vector::new(x, y, x+ax as i32, y+ay as i32, ACC_COLOR);
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>){
        let x = self.pos.x - (self.size/2) as i32;
        let y = self.pos.y - (self.size/2) as i32;
        let size = self.size;
        let rect = Rect::new(x,y,size,size);
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_rect(rect).expect("Error al dibujar la partícula");
        self.vel.draw(canvas);
        self.acc.draw(canvas);
        if self.graph_position{
            canvas.set_draw_color(Color::WHITE);
            canvas.draw_lines(self.position_reg[0..self.saved_points].as_ref()).expect("Error al dibujar la linea de trayectoria");
        }
    }
    pub fn add_to_trail(&mut self) {
        if self.graph_position{
            if self.saved_points == MAX_SAVED_POINTS {
                self.position_reg.rotate_left(1);
                self.position_reg[MAX_SAVED_POINTS - 1] = self.pos;
                return;
            }
            self.position_reg[self.saved_points] = self.pos;
            self.saved_points += 1;
        }
    }
    pub fn get_pos(&self) -> (f64,f64) {
        return (self.x, self.y);
    }
    pub fn apply_force(&mut self, fx: f64,fy:f64) -> &mut Self{
        self.ax += fx/(self.size as f64);
        self.ay += fy/(self.size as f64);
        self
    }
    pub fn angle_to_point(&self, x: f64, y: f64) -> f64 {       
        let x = self.x - x;
        let y = self.y - y;
        let mut result = (-y/x).atan();
        if x < 0.0{
            result += PI;
        }
        result
    }
    pub fn angle_and_distance(&self, x: f64, y: f64) -> (f64,f64) {       
        let x = self.x - x;
        let y = self.y - y;
        let mut result = (-y/x).atan();
        if x < 0.0{
            result += PI;
        }
        let distance = (x.powi(2)+y.powi(2)).sqrt();
        (result,distance)
    }
    pub fn get_size(&self) -> u32 {
        self.size
    }
}
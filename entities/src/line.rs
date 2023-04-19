use std::f64::consts::PI;

use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Clone, Copy)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self{
        let p1 = Point::new(x0, y0);
        let p2 = Point::new(x1, y1);
        let new_line = Line{start: p1,
                                  end: p2,
        };

        new_line
    }

    pub fn get_points(&self) -> (Point, Point){
        (self.start, self.end)
    }
    pub fn get_angle(&self) -> f64{
        let x = (self.end.x - self.start.x) as f64;
        let y = (self.end.y - self.start.y) as f64;
        let mut result = (-y/x).atan();
        if x < 0.0{
            result += PI;
        }
        result
    }
    pub fn draw_line(&self, canvas: &mut Canvas<Window>){
        canvas.draw_line(self.start,self.end).expect("Error al dibujar linea");
    }
}
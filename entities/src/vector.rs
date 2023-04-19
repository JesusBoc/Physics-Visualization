use std::f64::consts::PI;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

use crate::line::Line;

#[derive(Clone, Copy)]
pub struct Vector{
    center: Line,
    arrow: [Line; 2],
    color: Color,
}

impl Vector {
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32, color: Color) -> Self {
        let center = Line::new(x0,y0,x1,y1);
        let arrow = Self::get_arrow_lines(&center);
        let new_vector = Vector{
            center,
            arrow,
            color,
        };
        new_vector
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.color);
        self.center.draw_line(canvas);
        self.arrow[0].draw_line(canvas);
        self.arrow[1].draw_line(canvas);
    }
    
    fn get_arrow_lines(center: &Line) -> [Line; 2]{
        let angle = center.get_angle()+3.0*PI/4.0;
        let (_start, end) = center.get_points();
        let x_0 = ((end.x as f64) +  angle.cos()*20.0)as i32;
        let y_0 = ((end.y as f64) -  angle.sin()*20.0)as i32;
        let x_1 = ((end.x as f64) +  (angle + PI/2.0).cos()*20.0)as i32;
        let y_1 = ((end.y as f64) -  (angle + PI/2.0).sin()*20.0)as i32;
        let new_line = Line::new(end.x, end.y, x_0 ,y_0);
        let new_line2 = Line::new(end.x, end.y, x_1 ,y_1);
        [new_line,new_line2]
    }
}
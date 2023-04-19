use std::env;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use entities::particle::Particle;
use entities::particle::ParticleBuilder;
use std::sync::mpsc;
use std::thread;

pub const WINDOW_WIDTH: u32 = 960*2;
pub const WINDOW_HEIGHT: u32 = 540*2-100;
const VERTICAL_CENTER: f64 = (WINDOW_HEIGHT/2) as f64;
const HORIZONTAL_CENTER: f64 = (WINDOW_WIDTH/2) as f64;
//Em segundos

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 1 {
        println!("Uso: cargo run");
        return;
    }
    let (tx1,rx1) = mpsc::channel();
    let (tx2,rx2) = mpsc::channel();
    let (mut canvas, mut event_pump, mut pause) = init();

    thread::spawn(move || {
        println!("Elapsed,Simulated");
        loop {
            let (mut scene, pause): (Vec<Particle>,bool) = rx2.recv().unwrap();
            if pause{
                tx1.send(scene).unwrap();
                continue;
            }
            for _ in 0..16 {
                let mut indexes = Vec::new();
                let mut i: usize = 0;
                for particle in scene.as_mut_slice(){
                    let (angle,distance) = particle.angle_and_distance(HORIZONTAL_CENTER, VERTICAL_CENTER);
                    if (distance as u32) < particle.get_size() {
                        indexes.push(i);
                        continue;
                    }
                    let (fx, fy) = (-9.810*14.0*angle.cos()/(distance/1000.0).powi(2),9.810*14.0*angle.sin()/(distance/1000.0).powi(2));
                    particle.apply_force(fx,fy).update();
                    i += 1;
                }
                for index in indexes{
                    scene.remove(index);
                }
            }
            for particle in scene.as_mut_slice(){
                particle.add_to_trail();
            }
            tx1.send(scene).unwrap();
        }
    });
    
    let mut scene = reset_scene();
    draw(&mut canvas, &mut scene);
    tx2.send((scene,pause)).unwrap();
    'gameloop: loop {
        let mut scene = rx1.recv().unwrap();
        for evt in event_pump.poll_iter(){
            match evt {

                Event::Quit { .. } => {
                    break 'gameloop;
                },

                Event::KeyDown {keycode, ..} => {
                    match keycode {
                        Some(Keycode::Space) => {
                            pause = !pause;
                        },
                        Some(Keycode::R) => {
                            scene = reset_scene();
                            pause = false;
                        },
                        _ => (),
                    }
                },

                Event::MouseButtonDown { x, y, .. } => {
                    let new_particle = ParticleBuilder::new(WINDOW_HEIGHT, WINDOW_WIDTH)
                                                            .set_pos(x as f64, y as f64)
                                                            .set_vy(250.)
                                                            .track_position(true)
                                                            .build();
                    scene.push(new_particle);
                }
                _ => (),
            }
        }
        draw(&mut canvas, &mut scene);
        tx2.send((scene,pause)).unwrap();
        thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn init() -> (Canvas<Window>, sdl2::EventPump, bool) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Test 1", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();
    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump, false)
}
fn draw(canvas: &mut Canvas<Window>, scene: &mut Vec<Particle>){
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for particle in scene.as_mut_slice(){
        particle.draw(canvas);
    }
    canvas.present();
}
fn reset_scene() -> Vec<Particle> {
    let scene: Vec<Particle> = Vec::new();
    scene
}

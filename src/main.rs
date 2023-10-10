use std::fs::File;
use std::io::Write;
use std::string;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, self},
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
}

impl State{
    async fn new(window: Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
    

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())            
        .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let modes = &surface_caps.present_modes;

        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
        }


    }

    pub fn window(&self) -> &Window{
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
        todo!()
    }

    fn input(&mut self, event: &WindowEvent){
        todo!()
    }

    fn update(&mut self){
        todo!()
    }

    fn render(&mut self)-> Result<(), wgpu::SurfaceError>{
        todo!()
    }
}


fn main() {
    //timer 
    // let now = std::time::Instant::now();
    // let a = mandel_brot(1920,1080);
    // println!("Time elapsed: {:?}", now.elapsed());
    // to_ppm(a);
    // println!("Time elapsed: {:?}", now.elapsed());


    // wgpu hello world
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = State::new(window).await();

    
    
}

#[derive(Debug, Copy, Clone)]
struct Color{
    r: u32,
    g: u32,
    b: u32,
}

fn to_ppm(array:Vec<Vec<Color>>){
    let mut file = File::create("image.ppm").expect("Unable to create file");
    let header = format!("P3\n{} {}\n255\n", array[0].len(), array.len());
    file.write_all(header.as_bytes()).expect("Unable to write data");
    let mut buffer: string::String = String::new();
    for y in 0..array.len(){
        for x in 0..array[0].len(){
            buffer = buffer + &format!("{} {} {}\n", array[y][x].r, array[y][x].g, array[y][x].b)
        }
    }
    file.write_all(buffer.as_bytes()).expect("Unable to write data");


}


fn mandel_brot(screen_x: usize, screen_y:usize) -> Vec<Vec<Color>>{
    /*
        screen_x = width of screen
        screen_y = height of scren
        x0= scale of the mandelbrot
        y0 = scale of the mandelbrot
    */
    let mut array = vec![vec![Color{r:0, g:0, b:0};screen_x];screen_y];    

    for y in 0..screen_y {
        for x in 0..screen_x {
            let x0 = (x as f64 / screen_x as f64) * 3.5 - 2.5; 
            let y0 = (y as f64 / screen_y as f64) * 2.0 - 1.0;
            

            let mut x1 = 0.0;
            let mut y1 = 0.0;
            let mut iteration = 0;
            let max_iteration = 20;
            while x1*x1 + y1*y1 <= 2.0*2.0 && iteration < max_iteration{
                let xtemp = x1*x1 - y1*y1 + x0;
                y1 = 2.0*x1*y1 + y0;
                x1 = xtemp;
                iteration += 1;
            }
            if iteration == max_iteration{
                array[y][x] = Color{r:0, g:0, b:0};
            }
            else{
                let color = iteration * 255 / max_iteration;
                array[y][x] = Color{r:0, g:0, b:color};
            }
            
        }
    }
    return array;
    
}
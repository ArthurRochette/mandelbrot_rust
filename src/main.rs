
mod window;
use window::run;
mod mandelbrot;
use mandelbrot::mandel_brot;

fn main() {
    //timer 
    // let now = std::time::Instant::now();
    // let a = mandel_brot(1920,1080);
    // println!("Time elapsed: {:?}", now.elapsed());
    // to_ppm(a);
    // println!("Time elapsed: {:?}", now.elapsed());

    // wgpu hello world
    pollster::block_on(run());

    
}

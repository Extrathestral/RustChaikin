use cairo::{ ImageSurface, Format, Context };
use std::fs::File;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f64,
    y: f64
}

fn trace_path(p_list: &Vec<Point>, context: &Context) {
    context.move_to(p_list[0].x, p_list[0].y);
    for i in &p_list[1..] {
        context.line_to(i.x, i.y);
    }
}

fn calc_r(p0: &Point, p1: &Point) -> Point
{
    return Point {x: (p0.x*0.75)+(p1.x*0.25), y: (p0.y*0.75)+(p1.y*0.25)}
}

fn calc_q(p0: &Point, p1: &Point) -> Point
{
    return Point {x: (p0.x*0.25)+(p1.x*0.75), y: (p0.y*0.25)+(p1.y*0.75)}
}

fn chaikin(p_list: &Vec<Point>) -> Vec<Point>
{
    let f_point = p_list[0].clone();
    let l_point = p_list[p_list.len()-1].clone();
    let mut out_vec = vec![f_point];
    for i in 1..p_list.len()-1
    {
        out_vec.push(calc_r(&p_list[i], &p_list[i-1]));
        out_vec.push(calc_q(&p_list[i+1], &p_list[i]));
    }
    out_vec.push(l_point);
    
    return out_vec;
}

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn't create surface");
    let context = Context::new(&surface).expect("Couldn't create context.");
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().expect("Could not paint.");
    context.set_source_rgb(1.0, 0.0, 0.0);
    context.set_line_width(10.0);
    let p_list = vec![Point {x:50.0, y:50.0}, Point {x:550.0, y:550.0}, Point {x: 550.0, y:200.0}];
    let t_list = vec![Point { x: 50.0, y: 50.0 }, Point { x: 175.0, y: 175.0 }, Point { x: 550.0, y: 287.5 }, Point { x: 550.0, y: 200.0 }];

    let mut c_list: Vec<Point> = p_list.clone();
    for i in 0..5
    {
        println!("Iteration: {}",i);
        c_list = chaikin(&c_list);
    }

    trace_path(&c_list, &context);
    context.stroke().expect("cum");
    let mut file = File::create("output.png").expect("Couldn't create file"); 
    surface.write_to_png(&mut file).expect("Couldn't write to png");
}
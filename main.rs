//  Plot-R a simple plotting program in Rust that uses gtk4-rs 

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Button, Orientation, glib};
use std::sync::{Arc,Mutex};
use cairo::Context;
use std::time::{Duration};

use std::cell::Cell;
use std::rc::Rc;
use glib::clone;

const M_PI: f64 = 3.14159265358979323846;

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

pub struct Rectangle {
    x1: f64,
    y1: f64,
    w: f64,  // width
    h: f64   // height
}
#[derive(Clone)]
pub struct PlotParams {
	pub margin_width: i32,
	pub top_label: String,
	pub right_label: String,
	pub bottom_label: String,
	pub left_label: String,
	pub x0_max: f64,
	pub x0_min: f64,
	pub y0_max: f64, 
	pub y0_min: f64,
	pub x_max: f64,
	pub x_min: f64,
	pub y_max: f64, 
	pub y_min: f64,
	pub num_x_ticks: f64,
	pub num_y_ticks: f64
}

impl PlotParams {
	
	fn set_x_max(&mut self, new_x_max: f64) {
        self.x_max = new_x_max;
    }
    
    fn set_y_max(&mut self, new_y_max: f64) {
        self.y_max = new_y_max;
    }
    
    fn set_x_min(&mut self, new_x_min: f64) {
        self.x_min = new_x_min;
    }
    
    fn set_y_min(&mut self, new_y_min: f64) {
        self.y_min = new_y_min;
    }
}
	

struct Curve{
	x_vec: Vec<f64>,
    y_vec: Vec<f64>,
    color: (f64, f64, f64)
}

struct Border {
    pl_grid: gtk::Grid,
    pl_parms: PlotParams,
}

impl Border {
    fn new() -> Arc<Mutex<Border>>  {
        let result = Arc::new(Mutex::new(Border{
            pl_grid: gtk::Grid::new(),
            pl_parms: PlotParams { 
					margin_width: 50,
					//top_label: String::from("This is the top label"),
			//  If there is no top label then the top label will not draw.
					top_label: String::from("My Cool Plot"),
					right_label: String::from("This is the right label"),
					bottom_label: String::from("This is the bottom label"),
					left_label: String::from("This is the left label"),
					x0_max: 100.0,
					x0_min: 0.0,
					y0_max: 1.0, 
					y0_min: -1.0,
					x_max: 100.0,
					x_min: 0.0,
					y_max: 1.00,
					y_min: -1.0,
					num_x_ticks: 10.0,
					num_y_ticks: 5.0	
				},
				            
		}));
		
        
             
        result
    }
   
    fn create_plot(&mut self) {
		
		self.pl_grid.set_column_spacing(0);
		self.pl_grid.set_row_spacing(0);

		let x_ticks = create_tick_positions(self.pl_parms.x_max, self.pl_parms.x_min, self.pl_parms.y_max, self.pl_parms.y_min, self.pl_parms.num_x_ticks, self.pl_parms.num_y_ticks,"x");
		let y_ticks = create_tick_positions(self.pl_parms.x_max, self.pl_parms.x_min, self.pl_parms.y_max, self.pl_parms.y_min, self.pl_parms.num_x_ticks, self.pl_parms.num_y_ticks,"y");
		
		let h_axis_b = Axis::new(100, self.pl_parms.margin_width, 11.0, false, false, false);
		let h_axis_t = Axis::new(100, self.pl_parms.margin_width, 11.0, false, false, true);
		let v_axis_l = Axis::new(self.pl_parms.margin_width, 100, 11.0, false, false, false);
		let v_axis_r = Axis::new(self.pl_parms.margin_width, 100, 11.0, false, false, false);
		let axis_x_b = create_axis_x_b(h_axis_b, x_ticks, self.pl_parms.bottom_label.clone(), self.pl_parms.margin_width as f64);
		let axis_x_t = create_axis_x_t(h_axis_t, self.pl_parms.top_label.clone(), self.pl_parms.margin_width as f64);
		let axis_y_l = create_axis_y_l(v_axis_l, y_ticks, self.pl_parms.left_label.clone(), self.pl_parms.margin_width as f64);
		let axis_y_r = create_axis_y_r(v_axis_r, self.pl_parms.right_label.clone(), self.pl_parms.margin_width as f64);

		
		self.pl_grid.attach(&axis_x_t, 0, 0, 3, 1);
		self.pl_grid.attach(&axis_x_b, 0, 2, 3, 1);
		self.pl_grid.attach(&axis_y_l, 0, 0, 1, 3);
		self.pl_grid.attach(&axis_y_r, 2, 0, 1, 3);
		
	}
 
}


struct Canvas{
    draw_area: gtk::DrawingArea,
    x_axis_range: f64,
    y_axis_range: f64,
    y_axis_offset: f64,
    x_axis_offset: f64,
	curves: Vec<Curve>,
	selection: bool,
	rect: Rectangle

}

// Note that Canvas is of type Mutex.  This allows the dynamic update of
// members of the struct and their use in the draw function of the DrawingArea

impl Canvas {
    fn new() -> Arc<Mutex<Canvas>>  {
        let result = Arc::new(Mutex::new(Canvas{
            draw_area: create_canvas(),
            x_axis_range: 0.0,
            y_axis_range: 0.0,
            y_axis_offset: 0.0,
			x_axis_offset: 0.0,
            curves: Vec::new(),
            selection: false,
            rect: Rectangle {x1:0.0, y1:0.0, w:0.0, h:0.0}
            
        }));
        let r2 = result.clone();
        result.lock().unwrap().draw_area.set_draw_func(move|_, cr, w, h|{
           r2.lock().unwrap().redraw(cr, w, h);
        });
             
        result
    }
   
    fn set_y_axis_range(&mut self, new_range: f64) {
        self.y_axis_range = new_range;
    }
    
    fn set_x_axis_range(&mut self, new_range: f64) {
        self.x_axis_range = new_range;
    }
    
    fn set_x_axis_offset(&mut self, offset: f64) {
        self.x_axis_offset = offset;    
    }
    
    fn set_y_axis_offset(&mut self, offset: f64) {
        self.y_axis_offset = offset;    
    }
       
    fn add_curve (&mut self, new_curve: Curve){
		self.curves.push(new_curve);
	}
	
    fn redraw(&self, cr: &Context, w: i32, h: i32) {
		
		//interate through the curves and plot each one
		for ii in 0..self.curves.len(){
			
        cr.set_source_rgb(self.curves[ii].color.0, self.curves[ii].color.1, self.curves[ii].color.2);
				
		for i in 0..self.curves[ii].x_vec.len() {
			
			let x = (-self.x_axis_offset as f64 + self.curves[ii].x_vec[i])*w as f64/self.x_axis_range as f64;
			//let mut x = (self.x_axis_offset as f64 + self.x_vec[i])*w as f64/self.x_axis_range as f64;
			let y = h as f64 + ((self.y_axis_offset as f64 - self.curves[ii].y_vec[i])* h as f64)/self.y_axis_range as f64;
			//let mut y = h as f64 + ((self.y_axis_offset as f64 - self.y_vec[i])* h as f64)/self.y_axis_range as f64;
			if i == 0 {
				cr.move_to(x, y);
			} else {
				cr.line_to(x, y);
				let _res = cr.stroke();
				cr.move_to(x, y);
			}
			
// This is written to draw lines between points but the commented code shows how to print a character instead
			//let extents = cr.text_extents("x").unwrap(); 
			//x = x - (extents.width()/2.0);
			//y = y + (extents.height()/2.0);
			//cr.move_to(x, y);
			//let _res = cr.show_text("x");
			
		}		
			
		
			if self.selection {
				cr.rectangle(self.rect.x1, self.rect.y1, self.rect.w, self.rect.h);
				let _result = cr.stroke();
		}
			
		
		}
		()

	}	 
}

pub struct Axis {
	pub width: i32,
    pub height: i32,
    pub fontsz: f64,
    pub hstretch: bool,
    pub vstretch: bool,
    pub no_ticks: bool
}

impl Axis {

 fn new(w: i32, h: i32, fontsz: f64, hstr: bool, vstr: bool, no_t: bool ) -> Axis {
        Axis { width: w, height: h, fontsz: fontsz, hstretch: hstr, vstretch: vstr, no_ticks: no_t }			
	}
}

fn create_canvas() -> DrawingArea {
	let area = DrawingArea::new();
	area.set_content_width(300);
	area.set_content_height(200);
	area.set_hexpand(true);
	area.set_vexpand(true);
	area
}

// If I want to redraw the axes and tick marks I need to have them get the ticks[] from
// elsewhere than passing as a variable so that they can be changed on the fly.

fn create_axis_x_b(axis:Axis, ticks:Vec<f64>, label: String, m_width: f64) -> DrawingArea {
		let axis_x_b = DrawingArea::new();
		axis_x_b.set_content_width(axis.width);
		axis_x_b.set_content_height(axis.height);
		axis_x_b.set_vexpand(axis.vstretch);
		axis_x_b.set_hexpand(axis.hstretch);
		axis_x_b.set_draw_func(move|_, cr, w, h| {

			cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            let mut _res = cr.paint();
 
// draw line along bottom axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width, 1.0);
			cr.line_to(w as f64 - m_width, 1.0);
			_res = cr.stroke();
			
// insert the axis label 			
			cr.set_source_rgb(0.0, 0.0, 0.0);
			let extents2 = cr.text_extents(&label).unwrap();
			cr.move_to(w as f64/2.0 - extents2.width()/2.0, h as f64 - 7.0);
			_res = cr.show_text(&label);
			
//  Insert the tick marks...			
			for n in 0..ticks.len() {
				let x_factor = (w as f64 - 2.0*m_width)/(ticks.len() -1) as f64;
				let x = m_width + n as f64*x_factor;
				cr.move_to (x, 1.0);
				cr.line_to (x, 6.0);
				_res = cr.stroke();
// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", ticks[n]);
				let extents = cr.text_extents(&tick_label).unwrap();
				let x_tick_text = x - (extents.width()/2.0);
				cr.move_to (x_tick_text, 7.0 + extents.height());
				_res = cr.show_text(&tick_label);
// draw minor ticks...  only on the bottom axis
				let minor_ticks = x_factor/4.0;
				for i in 1..4 {	
					cr.move_to (x + i as f64*minor_ticks, 1.0);
					cr.line_to (x + i as f64*minor_ticks, 5.0);
				}			
			}	
			()
        });
        
	axis_x_b
}

fn create_axis_x_t(axis:Axis, label: String, m_width: f64) -> DrawingArea {
		let axis_x_t = DrawingArea::new();
		axis_x_t.set_content_width(axis.width);
		axis_x_t.set_content_height(axis.height);
		axis_x_t.set_vexpand(axis.vstretch);
		axis_x_t.set_hexpand(axis.hstretch);
		axis_x_t.set_draw_func(move|_, cr, w, _| {
			
			cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
			let mut _res = cr.paint();
			
// draw line along top axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width, m_width-1.0);
			cr.line_to(w as f64 - m_width, m_width-1.0);
			_res = cr.stroke();
			
// If there is no label do not draw label
// Label has larger text and lower position to make it a label for the plot rather than the axis
			if label.len() > 0 {
					// insert the axis label 			
				cr.set_source_rgb(0.0, 0.0, 0.0);
				let extents2 = cr.text_extents(&label).unwrap();
				cr.move_to(w as f64/2.0 - extents2.width()/2.0, extents2.height() + 8.0);
				_res = Ok(cr.set_font_size(15.0));
				_res = cr.show_text(&label);
				
			}
			
			if axis.no_ticks {  //do not draw ticks, tick_labels, or minor ticks
				return ();
			}


			for n in 0..11 {
//draw tick marks onlong the top axis
				let mut x = m_width + n as f64*(w as f64 - 2.0*m_width)/10.0;
				cr.move_to (x, m_width);
				cr.line_to (x, m_width -6.0);
				let mut _res = cr.stroke();
// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", n);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				x = x - (extents.width()/2.0);
				cr.move_to (x, m_width - extents.height() - 1.0 );
				_res = cr.show_text(&tick_label);			
			}
			()
        });
	axis_x_t
}

fn create_axis_y_l(axis:Axis, ticks:Vec<f64>, label: String, m_width: f64) -> DrawingArea {
	let axis_y_l = DrawingArea::new();
		axis_y_l.set_content_width(axis.width);
		axis_y_l.set_content_height(axis.height);
		axis_y_l.set_vexpand(axis.vstretch);
		axis_y_l.set_hexpand(axis.hstretch);
		axis_y_l.set_draw_func(move|_, cr, w, h| {
			
			cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            let mut _res = cr.paint();
            
// draw line along left axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width - 1.0, m_width);
			cr.line_to(m_width - 1.0, h as f64 - m_width);
			_res = cr.stroke();
			
// insert the tick marks and tick labels
			for n in 0..ticks.len(){
				//draw tick marks onlong the left axis
				let mut y = m_width + n as f64*(h as f64 - 2.0*m_width) /(ticks.len() -1) as f64;
				cr.move_to (m_width, y);
				cr.line_to (m_width -6.0, y);
				let mut _res = cr.stroke();
				// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", ticks[ticks.len() -1 - n]);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				y = y + (extents.height()/2.0);
				cr.move_to (m_width -7.0 - extents.width(), y);
				_res = cr.show_text(&tick_label);
			}
// insert the axis label			
			//cr.set_font_size(axislab.fontsz);
			let extents3 = cr.text_extents(&label).unwrap();
			let y = h/2 + (extents3.width()/2.0) as i32;
			let x = w/2 + (extents3.height()/2.0) as i32;
			cr.move_to(x as f64 -7.0, y as f64);
            cr.translate((h/2) as f64, (w/2) as f64);
			cr.rotate(-1.57);
            let _res4 = cr.show_text(&label);
			
			()
        });
	axis_y_l
}

fn create_axis_y_r(axis:Axis, label: String, m_width: f64) -> DrawingArea {
	let axis_y_r = DrawingArea::new();
		axis_y_r.set_content_width(axis.width);
		axis_y_r.set_content_height(axis.height);
		axis_y_r.set_vexpand(axis.vstretch);
		axis_y_r.set_hexpand(axis.hstretch);
		axis_y_r.set_draw_func(move|_, cr, w, h| {
			
			cr.set_source_rgba(0.0, 0.0, 0.0, 0.0);
            let mut _res =cr.paint();
            
// draw line along right axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(1.0, m_width);
			cr.line_to(1.0, h as f64 - m_width);
			_res = cr.stroke();
				
			for n in 0..11 {
//draw tick marks onlong the right axis
				let mut y = m_width + n as f64*(h as f64 - 2.0*m_width) /10.0;
				cr.move_to (0.0, y);
				cr.line_to (5.0, y);
				let mut _res = cr.stroke();
// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", 10-n);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				y = y + (extents.height()/2.0);
				cr.move_to (7.0, y);
				_res = cr.show_text(&tick_label);	
			}
// insert the axis label			
			//cr.set_font_size(axislab.fontsz);
			let extents3 = cr.text_extents(&label).unwrap();
			let y = h/2 + (extents3.width()/2.0) as i32;
			let x = w/2 + (extents3.height()/2.0) as i32;
			cr.move_to(x as f64 +7.0, y as f64);
            cr.translate((h/2) as f64, (w/2) as f64);
			cr.rotate(-1.57);
            let _res4 = cr.show_text(&label);
					
			
			()
        });
	axis_y_r
}

//  The following functions are all dealing with the generation of tick mark coordinates
//  the code is translated to Rust from JavaScript
// See https://github.com/cenfun/nice-ticks/ for the js code

fn f_int(n: f64) -> String {
    let s = n.to_string();
    let a: String = s.split(".").map(str::to_string).collect();
    return a;
}

fn f_len(n: f64) -> f64 {
    let s = n.to_string();
    let a: Vec<String> = s.split(".").map(str::to_string).collect();
    if a.len() > 1 {
        return a[1].len() as f64;
    }
    0.0
}

fn mul(n1: f64, n2: f64) -> f64{
    let r1 = f_len(n1);
    let r2 = f_len(n2);
    if r1 + r2 == 0.0 {
        return n1 * n2;
    }
    let m1: f64 = f_int(n1).parse().unwrap();
    let m2: f64 = f_int(n2).parse().unwrap();
    return (m1 * m2) as f64 / 10_f64.powf(r1 + r2);
}

fn nice(x: f64, round: bool) -> f64 {
	
    let exp: f64 = (x.ln().floor() / 10_f64.ln()).floor();
    let f = x / 10_f64.powf(exp);
    let nf;
    if round{
        if f < 1.5 {
            nf = 1.0;
        } else if f < 3.0 {
            nf = 2.0;
        } else if f < 7.0 {
            nf = 5.0;
        } else {
            nf = 10.0;
        }
    } else {
        if f <= 1.0 {
            nf = 1.0;
        } else if f <= 2.0 {
            nf = 2.0;
        } else if f <= 5.0 {
            nf = 5.0;
        } else {
            nf = 10.0;
        }
    }
    return nf * 10_f64.powf(exp);
    
}

fn myadd(n1: f64, n2: f64) -> f64 {
    let r1 = f_len(n1);
    let r2 = f_len(n2);
    if (r1 + r2) == 0.0 {
        return (n1 + n2).round();
    }
    let m = 10_f64.powf(f64::max(r1, r2));
    return ((n1 * m).round() + (n2 * m).round())/ m;
    }

//fn create_tick_positions(pl_params: &PlotParams, which: &str) -> Vec<f64>  {
fn create_tick_positions(x_max: f64, x_min: f64, y_max: f64, y_min: f64, num_x_ticks: f64, num_y_ticks: f64, which: &str) -> Vec<f64>  {
    
    let mut max: f64 = x_max;
	let mut min: f64 = x_min;
	let mut num: f64 = num_x_ticks;
	
	if which == "y" {
		max = y_max;
		min = y_min;
		num = num_y_ticks;
	}
         
    if min == max {
        max = min + 1.0;
    } else if min > max {
        let n = min;
        min = max;
        max = n;
    }
    let r = nice(max - min, false);
    let d = nice(r / (num - 1.0), true);
    let s = mul((min / d).floor(), d);
    let e = mul((max / d).ceil(), d);
    let mut arr = Vec::new();
    let mut v = s;
    while v <= e {
        arr.push(v);
        v = myadd(v, d);      
    }
    
    arr
}


fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn axis_range_setup(a2: &Arc<Mutex<Canvas>>, pl_params: &PlotParams) {
	
	let x_ticks = create_tick_positions(pl_params.x_max, pl_params.x_min, pl_params.y_max, pl_params.y_min, pl_params.num_x_ticks, pl_params.num_y_ticks,"x");
	let y_ticks = create_tick_positions(pl_params.x_max, pl_params.x_min, pl_params.y_max, pl_params.y_min, pl_params.num_x_ticks, pl_params.num_y_ticks, "y");
	
	let x_tick_range = (x_ticks[x_ticks.len()-1] - x_ticks[0]).abs();
	let y_tick_range = (y_ticks[y_ticks.len()-1] - y_ticks[0]).abs();

	a2.lock().unwrap().set_x_axis_offset(x_ticks[0] as f64);
	a2.lock().unwrap().set_y_axis_offset(y_ticks[0] as f64);

	a2.lock().unwrap().set_x_axis_range(x_tick_range as f64);
	a2.lock().unwrap().set_y_axis_range(y_tick_range as f64);
	//println!("{:?}", x_ticks);
	//println!("{:?}", y_ticks);

}


fn build_ui(app: &Application) {
  
	let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
	let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
//  pl_parms_config contains all of the information for constructing the plot framework      
   
    let pl_parms_config = PlotParams {
		margin_width: 50,
		//top_label: String::from("This is the top label"),
//  If there is no top label then the top label and tick marks will not draw.
		top_label: String::from("My Cool Plot"),
		right_label: String::from("This is the right label"),
		bottom_label: String::from("This is the bottom label"),
		left_label: String::from("This is the left label"),
		//the primary configuration must have *0_max and *_max the same.
		x0_max: 100.0,
		x0_min: 0.0,
		y0_max: 1.0, 
		y0_min: -1.0,
		x_max: 100.0,
		x_min: 0.0,
		y_max: 1.0,
		y_min: -1.0,
		num_x_ticks: 10.0,
		num_y_ticks: 5.0	

	};
 

//  canvas contains the plot drawing area
    
    let canvas = Canvas::new();
	let a2 = canvas.clone();
	let border = Border::new();
	let b2 = border.clone();
	b2.lock().unwrap().pl_parms = pl_parms_config;
	
// create_plot does everything except the plot area	
	b2.lock().unwrap().create_plot();
	
	axis_range_setup(&a2, &b2.lock().unwrap().pl_parms);
//  put the drawingarea into the grid	

	b2.lock().unwrap().pl_grid.attach(&a2.lock().unwrap().draw_area, 1, 1, 1, 1);
//  put the grid into a box
	gtk_box.append(&b2.lock().unwrap().pl_grid);

	let coords = gtk::Label::new(None);	
		
	gtk_box.append(&coords);
    gtk_box.append(&button);
	
	// Create a click gesture for the right button for canceling the zoom
    let m_gesture = gtk::GestureClick::new();

    // Set the gestures button to the right mouse button (=3)
    m_gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_SECONDARY as u32);

    // Assign your handler to an event of the gesture (e.g. the `pressed` event)
    m_gesture.connect_pressed(clone!(@weak  a2, @weak b2 => move|m_gesture, _, _, _| {
       m_gesture.set_state(gtk::EventSequenceState::Claimed);
//        This uses the original parameters in pl_parms_config to un-zoom
//		let allocation = a2.lock().unwrap().draw_area.allocation();
//		let width = allocation.width();
//		let height = allocation.height();

		let x_max_temp = b2.lock().unwrap().pl_parms.x0_max;
		let x_min_temp = b2.lock().unwrap().pl_parms.x0_min;
		let y_max_temp = b2.lock().unwrap().pl_parms.y0_max;
		let y_min_temp = b2.lock().unwrap().pl_parms.y0_min;
		b2.lock().unwrap().pl_parms.x_max = x_max_temp;
		b2.lock().unwrap().pl_parms.x_min = x_min_temp;
		b2.lock().unwrap().pl_parms.y_max = y_max_temp;
		b2.lock().unwrap().pl_parms.y_min = y_min_temp;

        axis_range_setup(&a2, &b2.lock().unwrap().pl_parms); 
        b2.lock().unwrap().pl_grid.remove_column(0);
		b2.lock().unwrap().pl_grid.remove_column(1);
		b2.lock().unwrap().pl_grid.remove_column(2);
		b2.lock().unwrap().pl_grid.remove_row(0);
		b2.lock().unwrap().pl_grid.remove_row(1);
		b2.lock().unwrap().pl_grid.remove_row(2);
		b2.lock().unwrap().pl_grid.remove(&a2.lock().unwrap().draw_area);
		
		b2.lock().unwrap().create_plot();
		
		b2.lock().unwrap().pl_grid.attach(&a2.lock().unwrap().draw_area, 1, 1, 1, 1);
		// erase the rectangle
		a2.lock().unwrap().selection = false;
		a2.lock().unwrap().rect.x1 = 0.0;
		a2.lock().unwrap().rect.y1 = 0.0;
		a2.lock().unwrap().rect.w = 0.0;
		a2.lock().unwrap().rect.h = 0.0;
		a2.lock().unwrap().draw_area.queue_draw();
    }));
    
    a2.lock().unwrap().draw_area.add_controller(&m_gesture);

// use the motion controller event to get current position of the cursor
// and convert it to user coords to display below the plot.

    let ecm = gtk::EventControllerMotion::new();
    a2.lock().unwrap().draw_area.add_controller(&ecm);
 
    ecm.connect_motion(clone!(@weak  a2 => move|_ecm, x, y| {
	// to calculate the graph position of the mouse, convert screen coords to graph coords.
		let mw =  &a2.lock().unwrap();
		let allocation = mw.draw_area.allocation();
		let width = allocation.width();
		let height = allocation.height();
		let xar = mw.x_axis_range;
		let yar = mw.y_axis_range;
		let xao = mw.x_axis_offset;
		let yao = mw.y_axis_offset;
		let x_scaled = xao as f64 + x*xar as f64/width as f64;
		let y_scaled = yao as f64 + (height as f64-y)*yar as f64/height as f64;
		let coord_str = format!("x= {}, y= {}", f64::trunc(x_scaled*100.0)/100.0, f64::trunc(y_scaled*100.0)/100.0);
		coords.set_text(&coord_str[..]);
   
	}));
	
//  use drag gesture to draw a zoom rectangle on the canvas

	let gesture_drag = gtk::GestureDrag::new();
	a2.lock().unwrap().draw_area.add_controller(&gesture_drag);
	gesture_drag.set_exclusive(true);
	
	gesture_drag.connect_drag_begin(clone!(@weak  a2 => move |_, start_x, start_y| {
		 let mw =  &mut a2.lock().unwrap();
		 mw.rect.x1 = start_x;  // note that this data is in draw_area screen coords
		 mw.rect.y1 = start_y;
		 mw.selection = true;
	}));
	
	gesture_drag.connect_drag_update(clone!(@weak  a2 => move |_, mov_x, mov_y| {
		 let mw =  &mut a2.lock().unwrap();
		 mw.rect.w = mov_x;
		 mw.rect.h = mov_y;
		
	}));
// gesture.connect_drag_end will remove the rectangle and expand the graph
	gesture_drag.connect_drag_end(clone!(@weak  a2 => move |_, mov_x, mov_y| {
	// Zoom in on the selection rectangle	
	//  A problem.  rect.* are in screen coordinates not graph coordinates
	//  Convert them
		let allocation = a2.lock().unwrap().draw_area.allocation();
		let width = allocation.width();
		let height = allocation.height();
		
		let mut x1_unscaled = a2.lock().unwrap().rect.x1;
		let mut x2_unscaled = x1_unscaled + mov_x;
		
		if mov_x < 0.0 {
			x2_unscaled = a2.lock().unwrap().rect.x1;
			x1_unscaled = a2.lock().unwrap().rect.x1 + mov_x;
		}
		let mut y1_unscaled = a2.lock().unwrap().rect.y1;
		let mut y2_unscaled = y1_unscaled + mov_y;
		if mov_y > 0.0 {
			y2_unscaled = a2.lock().unwrap().rect.y1;
			y1_unscaled = a2.lock().unwrap().rect.y1 + mov_y;
		}
		if x1_unscaled < 0.0 { x1_unscaled = 0.0 }
		if x1_unscaled > width as f64 { x1_unscaled = width as f64 }
		if x2_unscaled < 0.0 { x2_unscaled = 0.0 }
		if x2_unscaled > width as f64 { x2_unscaled = width as f64 }
		
		if y1_unscaled < 0.0 { x1_unscaled = 0.0 }
		if y1_unscaled > height as f64 { y1_unscaled = height as f64 }
		if y2_unscaled < 0.0 { y2_unscaled = 0.0 }
		if y2_unscaled > height as f64 { y2_unscaled = height as f64 }
		
		let xar = a2.lock().unwrap().x_axis_range;
		let yar = a2.lock().unwrap().y_axis_range;
		let xao = a2.lock().unwrap().x_axis_offset;
		let yao = a2.lock().unwrap().y_axis_offset;
		let x1_scaled = xao as f64 + x1_unscaled*xar as f64/width as f64;
		let x2_scaled = xao as f64 + x2_unscaled*xar as f64/width as f64;
		let y1_scaled = yao as f64 + (height as f64-y1_unscaled)*yar as f64/height as f64;
		let y2_scaled = yao as f64 + (height as f64-y2_unscaled)*yar as f64/height as f64;
		
		b2.lock().unwrap().pl_parms.set_x_max(x2_scaled);
		b2.lock().unwrap().pl_parms.set_x_min(x1_scaled);
		b2.lock().unwrap().pl_parms.set_y_max(y2_scaled);
		b2.lock().unwrap().pl_parms.set_y_min(y1_scaled);
		
		axis_range_setup(&a2, &b2.lock().unwrap().pl_parms); 
		// remove the old graph and put in the zoomed graph	
		b2.lock().unwrap().pl_grid.remove_column(0);
		b2.lock().unwrap().pl_grid.remove_column(1);
		b2.lock().unwrap().pl_grid.remove_column(2);
		b2.lock().unwrap().pl_grid.remove_row(0);
		b2.lock().unwrap().pl_grid.remove_row(1);
		b2.lock().unwrap().pl_grid.remove_row(2);
		b2.lock().unwrap().pl_grid.remove(&a2.lock().unwrap().draw_area);
		
		b2.lock().unwrap().create_plot();
		b2.lock().unwrap().pl_grid.attach(&a2.lock().unwrap().draw_area, 1, 1, 1, 1);
		// erase the rectangle
		a2.lock().unwrap().selection = false;
		a2.lock().unwrap().rect.x1 = 0.0;
		a2.lock().unwrap().rect.y1 = 0.0;
		a2.lock().unwrap().rect.w = 0.0;
		a2.lock().unwrap().rect.h = 0.0;
		a2.lock().unwrap().draw_area.queue_draw();
	}));
       
	let number = Rc::new(Cell::new(1));

		button.connect_clicked(clone!(@weak number, @weak  a2 =>
			move |_| {
				if number.get() == 1
				{ number.set(0)}
				else
				{ number.set(1);
						
				}
		}));


    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();
	
	window.set_child(Some(&gtk_box));

    // Present window
    window.show();
	let mut n = 0;
    
   
//  tick is a function called by the timer (see below).  Each time it is called 
// it calculates a sin function that is transferred to the MyWidget struct and that
// triggers a redraw of the plotting portion of the image.
// Each time it is called the function draws at a different displacement.  This gives
// the impression that the plot is moving.

	let tick = move || -> gio::prelude::Continue {
// if number is set to 1 then the plot is frozen in its current state.
		if number.get() == 0 { 
			a2.lock().unwrap().draw_area.queue_draw();
			return gio::prelude::Continue(true)
			}
        n += 1;
        if n > 500 { n = 1}
        a2.lock().unwrap().curves.clear();
        // animate the sequential plots by shifting the ydata by a bit for each plot
        let mut x2_vec = Vec::new();
        let mut y2_vec = Vec::new();
        // create the x, y data for a plot
        for i in 1..500 {     
			x2_vec.push((4 as f64*M_PI*(i as f64)/499 as f64)*20.0);
			y2_vec.push((4 as f64*M_PI*(n as f64 + 4 as f64*i as f64)/499 as f64).sin());
		}
				
		let mut c1 = Curve{x_vec: x2_vec, y_vec: y2_vec, color: (1.0, 0.0, 0.0)};
		a2.lock().unwrap().add_curve(c1);
		
        // add a second curve  .. actually a straight line!		
		let mut x3_vec = Vec::new();
        let mut y3_vec = Vec::new();
		
		y3_vec.push(-0.5);
		y3_vec.push(0.5);
		x3_vec.push(10.0);
		x3_vec.push(80.0);
		
        c1 = Curve{x_vec: x3_vec, y_vec: y3_vec, color: (0.0, 0.0, 1.0)};
		a2.lock().unwrap().add_curve(c1);
        
        // trigger a redraw of the canvas
        a2.lock().unwrap().draw_area.queue_draw();
        
        // we could return glib::Continue(false) to stop our clock after this tick
       gio::prelude::Continue(true)
    };
    
// using timeout_add_local to run the fn tick at regular intervals
	glib::timeout_add_local(Duration::from_millis(10), tick);  // This works. gio::prelude::timeout_add does not.

}

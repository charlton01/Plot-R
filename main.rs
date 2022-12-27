//use std::cell::Cell;
//use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, Grid, Button, Orientation, glib};
use is_close::is_close;
use std::sync::{Arc,Mutex};
use cairo::Context;
use std::time::{Duration};

const M_PI: f64 = 3.14159265358979323846;

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

pub struct PlotParams {
	pub margin_width: i32,
	pub top_label: String,
	pub right_label: String,
	pub bottom_label: String,
	pub left_label: String,
	pub x_max: f64,
	pub x_min: f64,
	pub y_max: f64, 
	pub y_min: f64,
	pub num_x_ticks: f64,
	pub num_y_ticks: f64
}

struct MyWidget{
    widget: gtk::DrawingArea,
    x_axis_range: i32,
    y_axis_range: i32,
    y_axis_offset: i32,
    x_axis_offset: i32,
    x_vec: Vec<f64>,
    y_vec: Vec<f64>,

}

impl MyWidget {
    fn new() -> Arc<Mutex<MyWidget>>  {
        let result = Arc::new(Mutex::new(MyWidget{
            widget: create_canvas(),
            x_axis_range: 0,
            y_axis_range: 0,
            y_axis_offset: 0,
			x_axis_offset: 0,
            x_vec: Vec::new(),
            y_vec: Vec::new(),
            
           
        }));
        let r2 = result.clone();
        result.lock().unwrap().widget.set_draw_func(move|_, cr, w, h|{
           r2.lock().unwrap().redraw(cr, w, h);
        });
              
        result
    }
    fn set_x_vec(&mut self, new_vec: &Vec<f64>) {
        self.x_vec = new_vec.to_vec();
        self.widget.queue_draw();
        
    }
    
    fn set_y_vec(&mut self, new_vec: &Vec<f64>) {
        self.y_vec = new_vec.to_vec();
        self.widget.queue_draw();
        
    }
    
    fn set_y_axis_range(&mut self, new_range: i32) {
        self.y_axis_range = new_range;
        self.widget.queue_draw();
    }
    
    fn set_x_axis_range(&mut self, new_range: i32) {
        self.x_axis_range = new_range;
        self.widget.queue_draw();   
    }
    
    fn set_x_axis_offset(&mut self, offset: i32) {
        self.x_axis_offset = offset;    
    }
    
    fn set_y_axis_offset(&mut self, offset: i32) {
        self.y_axis_offset = offset;    
    }
    
    fn redraw(&self, cr: &Context, w: i32, h: i32) {
        //println!("in draw:data_range{:?}", self.x_axis_range);
        //println!("In draw: x_vec_new {:?}", self.x_vec);
        cr.set_source_rgb(1.0, 0.0, 0.0);
				
		for i in 0..self.x_vec.len() {
			let extents = cr.text_extents("O").unwrap(); 
			let mut x = self.x_axis_offset as f64 + self.x_vec[i]*w as f64/self.x_axis_range as f64;
			let mut y = self.y_axis_offset as f64 + h as f64 - self.y_vec[i]* h as f64/self.y_axis_range as f64;
			x = x - (extents.width()/2.0);
			y = y + (extents.height()/2.0);
			cr.move_to(x, y);
			let _res = cr.show_text("O");
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
   
}

impl Axis {

 fn new(w: i32, h: i32, fontsz: f64, hstr: bool, vstr: bool ) -> Axis {
        Axis { width: w, height: h, fontsz: fontsz, hstretch: hstr, vstretch: vstr }			
	}
}


fn create_plot(pl_params: &PlotParams) -> Grid {
	let pl_grid = Grid::new();
	pl_grid.set_column_spacing(0);
	pl_grid.set_row_spacing(0);

	let x_ticks = create_tick_positions(&pl_params, "x");
	let y_ticks = create_tick_positions(&pl_params, "y");
	//let area = create_canvas(&pl_params);
	
	let h_axis_b = Axis::new(100, pl_params.margin_width, 11.0, false, false);
	let h_axis_t = Axis::new(100, pl_params.margin_width, 11.0, false, false);
	let v_axis_l = Axis::new(pl_params.margin_width, 100, 11.0, false, false);
	let v_axis_r = Axis::new(pl_params.margin_width, 100, 11.0, false, false);
	let axis_x_b = create_axis_x_b(h_axis_b, x_ticks, pl_params.bottom_label.clone(), pl_params.margin_width as f64);
	let axis_x_t = create_axis_x_t(h_axis_t, pl_params.top_label.clone(), pl_params.margin_width as f64);
	let axis_y_l = create_axis_y_l(v_axis_l, y_ticks, pl_params.left_label.clone(), pl_params.margin_width as f64);
	let axis_y_r = create_axis_y_r(v_axis_r, pl_params.right_label.clone(), pl_params.margin_width as f64);

	//pl_grid.attach(&area, 1, 1, 1, 1);
	pl_grid.attach(&axis_x_t, 0, 0, 3, 1);
	pl_grid.attach(&axis_x_b, 0, 2, 3, 1);
	pl_grid.attach(&axis_y_l, 0, 0, 1, 3);
	pl_grid.attach(&axis_y_r, 2, 0, 1, 3);
	
	pl_grid
	
}

fn create_canvas() -> DrawingArea {
	let area = DrawingArea::new();
	area.set_content_width(300);
	area.set_content_height(200);
	area.set_hexpand(true);
	area.set_vexpand(true);
//	area.set_draw_func(move|_, cr, w, h| {
//		cr.set_source_rgb(1.0, 0.0, 0.0);
//		for i in 0..x_vec.len() {
//			let extents = cr.text_extents("O").unwrap(); 
//			let mut x = x_vec[i]*w as f64/x_axis_range;
//			let mut y = h as f64 - y_vec[i]* h as f64/y_axis_range;
//			x = x - (extents.width()/2.0);
//			y = y + (extents.height()/2.0);
//			cr.move_to(x, y);
//			let _res = cr.show_text("O");
//		}
//		()
//		
//	});	
	area
}

fn create_axis_x_b(axis:Axis, ticks:Vec<f64>, label: String, m_width: f64) -> DrawingArea {
		let axis_x_b = DrawingArea::new();
		axis_x_b.set_content_width(axis.width);
		axis_x_b.set_content_height(axis.height);
		axis_x_b.set_vexpand(axis.vstretch);
		axis_x_b.set_hexpand(axis.hstretch);
		axis_x_b.set_draw_func(move|_, cr, w, h| {

			cr.set_source_rgba(0.0, 1.0, 0.0, 0.3);
            cr.paint();
 
// draw line along bottom axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width, 1.0);
			cr.line_to(w as f64 - m_width, 1.0);
			let mut _res = cr.stroke();
			
// insert the axis label 			
			cr.set_source_rgb(0.0, 0.0, 0.0);
			let extents2 = cr.text_extents(&label).unwrap();
			cr.move_to(w as f64/2.0 - extents2.width()/2.0, h as f64 - 7.0);
			_res = cr.show_text(&label);
			
//  Insert the tick marks...			
			for n in 0..ticks.len() {
				let x_factor = (w as f64 - 2.0*m_width)/(ticks.len() -1) as f64;
				let mut x = m_width + n as f64*x_factor;
				cr.move_to (x, 1.0);
				cr.line_to (x, 6.0);
				_res = cr.stroke();
// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
//				let tick_label = format!("{}", n);
				let tick_label = format!("{}", ticks[n]);
//				let extents = cr.text_extents(&tick_label).unwrap(); 
				let extents = cr.text_extents(&tick_label).unwrap();
				let x_tick_text = x - (extents.width()/2.0);
				cr.move_to (x_tick_text, 7.0 + extents.height());
				_res = cr.show_text(&tick_label);
// draw minor ticks
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
			
			cr.set_source_rgba(0.0, 1.0, 0.0, 0.3);
			cr.paint();
			
// draw line along top axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width, m_width-1.0);
			cr.line_to(w as f64 - m_width, m_width-1.0);
			let mut _res = cr.stroke();
			
// If there is no label do not draw ticks, tick_labels, or minor ticks
			if label.len() == 0 {
				return ()
			}

// insert the axis label 			
			cr.set_source_rgb(0.0, 0.0, 0.0);
			let extents2 = cr.text_extents(&label).unwrap();
			cr.move_to(w as f64/2.0 - extents2.width()/2.0, extents2.height() + 2.0);
			_res = cr.show_text(&label);

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
			
			cr.set_source_rgba(0.0, 1.0, 0.0, 0.3);
            cr.paint();
            
// draw line along left axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(m_width - 1.0, m_width);
			cr.line_to(m_width - 1.0, h as f64 - m_width);
			let mut _res = cr.stroke();
			
// insert the tick marks and tick labels
//			for n in 0..11 {
			for n in 0..ticks.len(){
				//draw tick marks onlong the left axis
				let mut y = m_width + n as f64*(h as f64 - 2.0*m_width) /(ticks.len() -1) as f64;
				//if x == 20.0 { x = 21.0;}
				//if x >= (w - 20) as f64 {x = (w-20) as f64}
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
			
			cr.set_source_rgba(0.0, 1.0, 0.0, 0.3);
            cr.paint();
            
// draw line along right axis
			cr.set_source_rgb(0.0, 0.0, 0.0);
			cr.set_line_width(1.0);
			cr.move_to(1.0, m_width);
			cr.line_to(1.0, h as f64 - m_width);
			let mut _res = cr.stroke();
				
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
    let mut nf = 0.0;
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

fn create_tick_positions(pl_params: &PlotParams, which: &str) -> Vec<f64>  {
    
    let mut max: f64 = pl_params.x_max;
	let mut min: f64 = pl_params.x_min;
	let mut num: f64 = pl_params.num_x_ticks;
	
	if which == "y" {
		max = pl_params.y_max;
		min = pl_params.y_min;
		num = pl_params.num_y_ticks;
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
           
    let mut pl_params = PlotParams {
		margin_width: 40,
		//top_label: String::from("This is the top label"),
		top_label: String::from(""),
		right_label: String::from("This is the right label"),
		bottom_label: String::from("This is the bottom label"),
		left_label: String::from("This is the left label"),
		x_max: 100.0,
		x_min: 0.0,
		y_max: 10.0,
		y_min: -10.0,
		num_x_ticks: 10.0,
		num_y_ticks: 5.0	
	};
    
    

    let y_vec3 = vec![0.0, 1.0, 2.0, 2.0, 8.0, 8.0, 20.0, 40.0, 70.0];
    
    let mut pl_grid = create_plot(&pl_params);
    
    let my_widget = MyWidget::new();
	let a2 = my_widget.clone();
	
	let x_ticks = create_tick_positions(&pl_params, "x");
	let y_ticks = create_tick_positions(&pl_params, "y");
	
	println!("{}", x_ticks[x_ticks.len()-1]);
	
	let x_tick_range = (x_ticks[x_ticks.len()-1] - x_ticks[0]).abs();
	let y_tick_range = (y_ticks[y_ticks.len()-1] - y_ticks[0]).abs();
	
	a2.lock().unwrap().set_x_axis_offset(x_ticks[0] as i32);
	a2.lock().unwrap().set_y_axis_offset(y_ticks[0] as i32);
	
	a2.lock().unwrap().set_x_axis_range(x_tick_range as i32);
	a2.lock().unwrap().set_y_axis_range(y_tick_range as i32);
	
    pl_grid.attach(&a2.lock().unwrap().widget, 1, 1, 1, 1);
    gtk_box.append(&pl_grid);

    gtk_box.append(&button);

// Reference-counted object with inner mutability
//    let number = Rc::new(Cell::new(0));
    
	button.connect_clicked(move |_| {
		
            
     });


    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();
	
	window.set_child(Some(&gtk_box));

    // Present window
    window.show();
	let mut n = 0;
	let tick = move || -> gio::prelude::Continue {
        n += 1;
        if n > 100 { n = 1}
        let mut x2_vec = Vec::new();
        let mut y2_vec = Vec::new();     
        for i in 1..100 { 
		    
        x2_vec.push((4 as f64*M_PI*(i as f64)/99 as f64)*100.0);
        y2_vec.push((4 as f64*M_PI*(n as f64 + 4 as f64*i as f64)/99 as f64).sin());
       
      }
        a2.lock().unwrap().set_x_vec(&x2_vec);
       
        a2.lock().unwrap().set_y_vec(&y2_vec);
      
        // we could return glib::Continue(false) to stop our clock after this tick
       gio::prelude::Continue(true)
    };
    
// use timeout_add_local to run the fn tick at regular intervals
	glib::timeout_add_local(Duration::from_millis(100), tick);  // This works. gio::prelude::timeout_add does not.

    //window.present();
}

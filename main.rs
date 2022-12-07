use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, DrawingArea, Grid};
use is_close::is_close;
use std::process::exit;

//use gtk::Orientation::{Vertical, Horizontal};
//use pango::{FontDescription};
//use cairo::{FontSlant, FontWeight};

const APP_ID: &str = "org.gtk_rs.HelloWorld3";




pub struct AxisLabel {
	pub width: i32,
    pub height: i32,
    pub label: String,
    pub fontsz: f64,
    pub hstretch: bool,
    pub vstretch: bool,
}

impl AxisLabel {

 fn new(w: i32, h: i32, label: String, fontsz: f64, hstr: bool, vstr: bool ) -> AxisLabel {
        AxisLabel { width: w, height: h, label: label, fontsz: fontsz, hstretch: hstr, vstretch: vstr }
			
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

fn create_vlabel(axislab:AxisLabel)-> DrawingArea {
		let pl_label = DrawingArea::new();
		pl_label.set_content_width(axislab.width);
		pl_label.set_content_height(axislab.height);
		pl_label.set_vexpand(axislab.vstretch);
		pl_label.set_hexpand(axislab.hstretch);
		pl_label.set_draw_func(move|_, cr, w, h| {

            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_font_size(axislab.fontsz);
			let extents = cr.text_extents(&axislab.label).unwrap();
			let y = h/2 + (extents.width()/2.0) as i32;
			let x = w/2 + (extents.height()/2.0) as i32;
			cr.move_to(x as f64, y as f64);
            cr.translate((h/2) as f64, (w/2) as f64);
			cr.rotate(-1.57);
            let _res4 = cr.show_text(&axislab.label);
            ()
        });
        pl_label
}

fn create_hlabel(axislab:AxisLabel)-> DrawingArea {
		let pl_label = DrawingArea::new();
		pl_label.set_content_width(axislab.width);
		pl_label.set_content_height(axislab.height);
		pl_label.set_vexpand(axislab.vstretch);
		pl_label.set_hexpand(axislab.hstretch);
		pl_label.set_draw_func(move|_, cr, w, h| {

            cr.set_source_rgb(0.0, 0.0, 0.0);
            cr.set_font_size(axislab.fontsz);
			let extents = cr.text_extents(&axislab.label).unwrap();
			let y = h/2 + (extents.height()/2.0) as i32;
			let x = w/2 - (extents.width()/2.0) as i32;
			cr.move_to(x as f64, y as f64);
            cr.translate((h/2) as f64, (w/2) as f64);
			//cr.rotate(-1.57);
            let _res4 = cr.show_text(&axislab.label);
            ()
        });
        pl_label
}

fn create_axis_x_b(axis:Axis, ticks:Vec<f64>) -> DrawingArea {
	let axis_x_b = DrawingArea::new();
		axis_x_b.set_content_width(axis.width);
		axis_x_b.set_content_height(axis.height);
		axis_x_b.set_vexpand(axis.vstretch);
		axis_x_b.set_hexpand(axis.hstretch);
		axis_x_b.set_draw_func(move|_, cr, w, h| {
			
			// draw line along bottom axis
			cr.set_line_width(1.0);
			cr.move_to(20.0, 1.0);
			cr.line_to((w - 20) as f64, 1.0);
			let mut _res = cr.stroke();
			
			for n in 0..ticks.len() {
//			for n in 0..11 {
//				//draw tick marks onlong the bottom axis
//				let mut x = 20.0 + n as f64*(w - 40) as f64/10.0;
//				if x == 20.0 { x = 20.0;}
//				if x >= (w - 20) as f64 {x = (w-20) as f64}
//				println!("{}  {}", x, w);
				let mut x = 20.0 + n as f64*(w - 40) as f64/(ticks.len() -1) as f64;
				cr.move_to (x, 1.0);
				cr.line_to (x, 5.0);
				_res = cr.stroke();
				// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
//				let tick_label = format!("{}", n);
				let tick_label = format!("{}", ticks[n]);
//				let extents = cr.text_extents(&tick_label).unwrap(); 
				let extents = cr.text_extents(&tick_label).unwrap();
				x = x - (extents.width()/2.0);
				cr.move_to (x, 6.0 + extents.height());
				_res = cr.show_text(&tick_label);			
			}
			()
        });
	axis_x_b
}

fn create_axis_x_t(axis:Axis) -> DrawingArea {
	let axis_x_t = DrawingArea::new();
		axis_x_t.set_content_width(axis.width);
		axis_x_t.set_content_height(axis.height);
		axis_x_t.set_vexpand(axis.vstretch);
		axis_x_t.set_hexpand(axis.hstretch);
		axis_x_t.set_draw_func(move|_, cr, w, h| {
			
			// draw line along top axis
			cr.set_line_width(1.0);
			cr.move_to(20.0, 20.0);
			cr.line_to((w - 20) as f64, 20.0);
			let mut _res = cr.stroke();
			
			for n in 0..11 {
				//draw tick marks onlong the top axis
				let mut x = 20.0 + n as f64*(w - 40) as f64/10.0;
				//if x == 20.0 { x = 21.0;}
				//if x >= (w - 20) as f64 {x = (w-20) as f64}
				//println!("{}  {}", x, w);
				cr.move_to (x, 19.0);
				cr.line_to (x, 14.0);
				let mut _res = cr.stroke();
				// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", n);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				x = x - (extents.width()/2.0);
				cr.move_to (x, 13.0);
				_res = cr.show_text(&tick_label);			
			}
			()
        });
	axis_x_t
}

fn create_axis_y_l(axis:Axis) -> DrawingArea {
	let axis_y_l = DrawingArea::new();
		axis_y_l.set_content_width(axis.width);
		axis_y_l.set_content_height(axis.height);
		axis_y_l.set_vexpand(axis.vstretch);
		axis_y_l.set_hexpand(axis.hstretch);
		axis_y_l.set_draw_func(move|_, cr, w, h| {
			// draw line along left axis
			cr.set_line_width(1.0);
			cr.move_to(20.0, 19.0);
			cr.line_to(20.0, (h - 19) as f64);
			let mut _res = cr.stroke();
			for n in 0..11 {
				//draw tick marks onlong the left axis
				let mut y = 20.0 + n as f64*(h - 40) as f64/10.0;
				//if x == 20.0 { x = 21.0;}
				//if x >= (w - 20) as f64 {x = (w-20) as f64}
				//println!("{}  {}", x, w);
				cr.move_to (20.0, y);
				cr.line_to (14.0, y);
				let mut _res = cr.stroke();
				// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", 10-n);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				y = y + (extents.height()/2.0);
				cr.move_to (2.0, y);
				_res = cr.show_text(&tick_label);	
			}
			()
        });
	axis_y_l
}

fn create_axis_y_r(axis:Axis) -> DrawingArea {
	let axis_y_r = DrawingArea::new();
		axis_y_r.set_content_width(axis.width);
		axis_y_r.set_content_height(axis.height);
		axis_y_r.set_vexpand(axis.vstretch);
		axis_y_r.set_hexpand(axis.hstretch);
		axis_y_r.set_draw_func(move|_, cr, w, h| {
			// draw line along right axis
			cr.set_line_width(1.0);
			cr.move_to(1.0, 19.0);
			cr.line_to(1.0, (h - 19) as f64);
			let mut _res = cr.stroke();
			for n in 0..11 {
				//draw tick marks onlong the right axis
				let mut y = 20.0 + n as f64*(h - 40) as f64/10.0;
				//if x == 20.0 { x = 21.0;}
				//if x >= (w - 20) as f64 {x = (w-20) as f64}
				//println!("{}  {}", x, w);
				cr.move_to (0.0, y);
				cr.line_to (5.0, y);
				let mut _res = cr.stroke();
				// draw tick labels (numbers) at each tick mark
				cr.set_source_rgb(0.0, 0.0, 0.0);
				cr.set_font_size(axis.fontsz);
				let tick_label = format!("{}", 10-n);
				let extents = cr.text_extents(&tick_label).unwrap(); 
				y = y + (extents.height()/2.0);
				cr.move_to (6.0, y);
				_res = cr.show_text(&tick_label);	
			}
					
			
			()
        });
	axis_y_r
}

fn create_tick_positions(data:Vec<f64>) -> Vec<f64>  {

	 if data.len() == 0 {
        return [].to_vec()
	}
	let mut return_points: Vec<f64> = Vec::new();
	let mut max_x = data.iter().into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
	let mut min_x = data.iter().into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
	println!("{} {}", max_x, min_x);
	let max_x = 600.0;
	let min_x = 300.0;
	
	let data_range = max_x - min_x;
	let lower_bound = min_x - data_range/10.0;
    let upper_bound = max_x + data_range/10.0;
    let view_range: f64 = upper_bound - lower_bound;
    let mut num  = lower_bound;
	let logten = view_range.log10() - 1.0;
	let mut n = logten.floor();
    let mut interval = 10_f64.powf(n);
    let mut num_ticks = 1;
    println!("{} {} {} {}", lower_bound, upper_bound, data_range, n);
    while num <= upper_bound {
        num += interval;
        num_ticks += 1;
        if num_ticks > 10 {
            if interval == 10_f64.powf(n) {
                interval = 2.0 * 10_f64.powf(n);
            }
            else if interval == 2.0 * 10_f64.powf(n) {
                interval = 4.0 * 10_f64.powf(n);
			}
            else if interval == 4.0 * 10_f64.powf(n) {
                interval = 5.0 * 10_f64.powf(n);
			}
            else {
                n += 1.0;
                interval = 10_f64.powf(n);
                println!("In final else");
            }
            num = lower_bound;
            num_ticks = 1;
		}
	}
	let mut copy_interval = 0.0;
	println!("{} {} {} {}", num, num_ticks, interval, n);
	if view_range >= 10.0 {
			copy_interval = interval;
	}
    else if interval == 10_f64.powf(n) {
            copy_interval = 1.0;
		}
    else if interval == 2.0 * 10_f64.powf(n) {
            copy_interval = 2.0;
		}
        else if interval == 4.0 * 10_f64.powf(n) {
            copy_interval = 4.0;
		}
        else {
            copy_interval = 5.0;
		}
	println!("{} {} {} {} {} {}", view_range, num, num_ticks, interval, copy_interval, n);
	let mut first_val = 0.0;
    let mut prev_val = 0.0;
    let mut times = 0.0;
    let mut temp_log = interval.log10();
    if is_close!(lower_bound, 0.0) {
        first_val = 0.0;
	}
    else if lower_bound < 0.0 {
        if upper_bound < -2.0*interval {
            if n < 0.0 {
				let exp = temp_log.abs() +1.0;
				let ub2 = 10_f64.powf(exp);
				let copy_ub = (upper_bound*ub2).round();
//                let copy_ub = round(upper_bound*10**(abs(temp_log) + 1));
                times = copy_ub; // round(interval*10**(abs(temp_log) + 1)) + 2
			}
            else {
                times = upper_bound; // round(interval) + 2
			}
		}
        while first_val >= lower_bound {
            prev_val = first_val;
            first_val = times * copy_interval;
            if n < 0.0 {
                first_val *= 10_f64.powf(n);
			}
            times -= 1.0;
		}
        first_val = prev_val;
        times += 3.0;
	}
    else {
        if lower_bound > 2.0*interval {
            if n < 0.0 {
				let exp = temp_log.abs() +1.0;
				let ub2 = 10_f64.powf(exp);
				let copy_ub = (lower_bound*ub2).round();
 
//                copy_ub = round(lower_bound*10**(abs(temp_log) + 1));
                times = copy_ub; // round(interval*10**(abs(temp_log) + 1)) - 2
			}
            else{
                times = lower_bound; // round(interval) - 2
			}
		}
        while first_val < lower_bound {
            first_val = times*copy_interval;
            if n < 0.0 {
                first_val *= 10_f64.powf(n);
			}
            times += 1.0;
		}
	}
	
	let mut retpoints = Vec::new();
	if n < 0.0 {
        retpoints.push(first_val);
	}
    else {
        retpoints.push(first_val.round());
	}
    let mut val = first_val;
    times = 1.0;
    println!("{} {} {} {} {}", val, upper_bound, first_val, times, interval);
    while val <= upper_bound {
        val = first_val + times * interval;
        if n < 0.0 {
            retpoints.push(val);
		}
        else {
            retpoints.push(val.round());
		}
        times += 1.0;

	}
//    retpoints.pop();
    
    println!("{:?}", retpoints);
    
	retpoints	     

//	[].to_vec()
}

//fn transform_coords(x1: f64, y1: f64, sx: f64, sy: f64, xoffset: f64, yoffset: f64) -> (f64, f64) {
	
	
//	(1.0, 2.0)
//} 

//fn plot_graph(


fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}


fn build_ui(app: &Application) {

    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });
    
//    let box_main = Box::builder()
//		.build();
    
    let y_vec = vec![0.0, 1.0, 2.0, 4.0, 8.0, 10.0, 20.0, 40.0, 80.0];
    let x_vec = vec![0.0, 10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];
    
	let y2_vec = vec![300.0, 301.0, 350.0, 400.0, 450.0, 503.0, 599.0, 600.0];
	let ticks = create_tick_positions(y2_vec);


	let area = DrawingArea::new();
	area.set_content_width(100);
	area.set_content_height(100);
	area.set_hexpand(true);
	area.set_draw_func(move|_, cr, w, h| {
		cr.set_source_rgb(1.0, 0.0, 0.0);
		for i in 0..x_vec.len() {
//			println!("{}  {}", x_vec[i], -y_vec[i]+h as f64);
			let extents = cr.text_extents("O").unwrap(); 
			let mut x = x_vec[i]*w as f64/100.0;
			let mut y = h as f64 - y_vec[i]* h as f64/100.0;
			x = x - (extents.width()/2.0);
			y = y + (extents.height()/2.0);
			cr.move_to(x, y);
			let _res = cr.show_text("O");
		}
		()
	});	
        
	let v_label_l = AxisLabel::new(14,100,"This is a left axis label".to_string(), 11.0, false, true);
	let v_label_r = AxisLabel::new(14,100,"This is a right axis label".to_string(), 11.0, false, true);
	let h_label_b = AxisLabel::new(100,14,"This is a bottom axis label".to_string(), 11.0, true, false);
	let h_label_t = AxisLabel::new(100,14,"This is a top axis label".to_string(), 11.0, true, false);
	let ax_label_l = create_vlabel(v_label_l);
	let ax_label_r= create_vlabel(v_label_r);
	let ax_label_b = create_hlabel(h_label_b);
	let ax_label_t = create_hlabel(h_label_t);
	
	let h_axis_b = Axis::new(134, 20, 11.0, true, false);
	let h_axis_t = Axis::new(100, 20, 11.0, true, false);
	let v_axis_l = Axis::new(21, 100, 11.0, false, true);
	let v_axis_r = Axis::new(21, 100, 11.0, false, true);
	let axis_x_b = create_axis_x_b(h_axis_b, ticks);
	let axis_x_t = create_axis_x_t(h_axis_t);
	let axis_y_l = create_axis_y_l(v_axis_l);
	let axis_y_r = create_axis_y_r(v_axis_r);
	
		
//	pl_label.set_markup("<span foreground=\"red\" size=\"8000\"> <b>Probe-1</b> </span>");

	let pl_grid = Grid::new();
	pl_grid.set_column_spacing(0);
//	pl_grid.set_column_homogeneous(true);
//	pl_grid.set_row_homogeneous(true);
	pl_grid.set_row_spacing(0);
// grid.attach(&widget, column, row, width, height)
	
	pl_grid.attach(&ax_label_t, 2, 0, 1, 1);
	pl_grid.attach(&axis_x_t, 1, 1, 3, 1);
	pl_grid.attach(&area, 2, 2, 1, 1);
	pl_grid.attach(&ax_label_l, 0, 2, 1, 1);
	pl_grid.attach(&axis_y_l, 1, 1, 1, 3);
	pl_grid.attach(&axis_y_r, 3, 1, 1, 3);
	pl_grid.attach(&ax_label_r, 4, 2, 1, 1);
	pl_grid.attach(&axis_x_b, 1, 3, 3, 1);
	pl_grid.attach(&ax_label_b, 2, 4, 1, 1);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();
	
	window.set_child(Some(&pl_grid));

    // Present window

    window.present();
}

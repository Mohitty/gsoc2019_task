mod geometry;
extern crate nalgebra;

use nalgebra::*;
use geometry::*;

fn main(){
	//let plane_num = 3;

	// Plane definition: 
	//arguments = (1) center of plane, (2) local x axis, 
	//			  (3) local y axis, (4) dimension along local x axis,
	//			  (5) dimension along local y axis.
  	let plane1 = Plane::new(Vector3::new(0.0,0.0,1.0),
  							Vector3::new(1.0, 0.0, 0.0),
  							Vector3::new(0.0,1.0,0.0),
  							5.0, 5.0);

  	//let mut plane2 = plane::new(Vector3::new(0.0,0.0,2.0),Vector3::new(0.707,0.707,0.0),Vector3::new(-0.707,0.707,0.0),5.0,5.0);
  	//let mut plane3 = plane::new(Vector3::new(0.0,0.0,3.0),Vector3::new(0.0, 1.0, 0.0),Vector3::new(-1.0,0.0,0.0),5.0,5.0);

  	let local_pt = Vector3::new(10.0,1.0,2.0);
   	println!("local point on plane {}", local_pt);

  	println!("Point lies inside the bounds of plane? {:?}", plane1.is_inside(local_pt));

  	let gl_point = plane1.local_to_global(local_pt);
   	println!("local point transformed to global point {}", gl_point);


  	let point = plane1.global_to_local(gl_point);
  	println!("global point transformed back to local point {}", point);
}
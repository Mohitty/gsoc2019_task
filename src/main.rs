mod geometry;
extern crate nalgebra;

use nalgebra::*;
use geometry::*;

fn main(){
    let bounds = RectangularBounds{width: 5.0, height: 5.0};
  	let plane1 = Plane::new(Vector3::new(0.0,0.0,1.0),
  							Vector3::new(1.0, 0.0, 0.0),
  							Vector3::new(0.0,1.0,0.0),
  							bounds);
    
  	//let plane1= Plane::new(Vector3::new(0.0,0.0,2.0),Vector3::new(0.707,0.707,0.0),Vector3::new(-0.707,0.707,0.0),bounds);
  	//let plane3 = Plane::new(Vector3::new(0.0,0.0,3.0),Vector3::new(0.0, 1.0, 0.0),Vector3::new(-1.0,0.0,0.0),bounds);

  	let local_pt = Vector3::new(1.0,1.0,1.0);
   	println!("local point on plane {}", local_pt);

  	let global_point = plane1.local_to_global(local_pt);
   	println!("local point transformed to global point {}", global_point);

    println!("Point lies inside the bounds of plane? {:?}", plane1.is_inside(global_point));

  	let point = plane1.global_to_local(global_point);
  	println!("global point transformed back to local point {}", point);
}
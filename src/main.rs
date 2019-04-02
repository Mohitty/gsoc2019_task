mod geometry;
extern crate nalgebra;

use nalgebra::*;
use geometry::*;

fn main(){
    let bounds = RectangularBounds{width: 5.0, height: 5.0};

    let m = Matrix4::new(1.0,0.0,0.0,0.0,
                         0.0,1.0,0.0,0.0,
                         0.0,0.0,1.0,1.0,
                         0.0,0.0,0.0,1.0);
    let t :Transform3<f32> = Transform3::from_matrix_unchecked(m);

    let plane1 = Plane::new(t,bounds);
  	
  	let local_pt = Point3::new(10.0,1.0,2.0);
   	println!("local point in plane coordinates {}", local_pt);

    //to check whether the local point lies inside the bounds of the plane
    println!("Local point lies inside the bounds of plane ?  {:?}", plane1.bounds.inside_bounds(local_pt));

    //transform the point from local coordinate system to global system
  	let global_point = plane1.local_to_global(local_pt);
   	println!("local point transformed to global point {}", global_point);

    //to check whether the global point lies inside the bounds of the plane
    println!("Global point lies inside the bounds of plane? {:?}", plane1.is_inside(global_point));

    //transform the point from global coordinate system to local coordinate system
  	let point = plane1.global_to_local(global_point);
  	println!("global point transformed back to local point {}", point);
}

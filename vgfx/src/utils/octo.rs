use std::clone;
use std::rc::Rc;


const DEPTH_SIZE: u32 = 5;
const MAX_SIZE: f32 = 50000.0;

pub struct OctTree<T> {
    root: CubeTree<T>,
    max_depth: u32,
    max_size: f32,
    cnt: u32
}

enum CubePosition {
    None,
    TopLeftFront,
    TopLeftBack,
    TopRightFront,
    TopRightBack,
    BotLeftFront,
    BotLeftBack,
    BotRightFront,
    BotRightBack
}

struct CubeSet {
    w: f32,
    h: f32,
    d: f32,
    mid_x: f32,
    mid_y: f32,
    mid_z: f32
}

struct CubeTree<T> {
    topLeftFront: Option<Box<CubeTree<T>>>,
    topLeftBack: Option<Box<CubeTree<T>>>,
    topRightFront: Option<Box<CubeTree<T>>>,
    topRightBack: Option<Box<CubeTree<T>>>,

    botLeftFront: Option<Box<CubeTree<T>>>,
    botLeftBack: Option<Box<CubeTree<T>>>,
    botRightFront: Option<Box<CubeTree<T>>>,
    botRightBack: Option<Box<CubeTree<T>>>,

    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    z1: f32,
    z2: f32,
    
    payload: Option<Vec<Box<T>>>
}

impl<T> OctTree<T> where T: Clone {

    pub fn new() -> Self {
        return OctTree { 
            root: CubeTree::new(0.0, 0.0, 0.0, MAX_SIZE, MAX_SIZE, MAX_SIZE, 0, DEPTH_SIZE),
            max_depth: DEPTH_SIZE,
            max_size: MAX_SIZE,
            cnt: 0
         }
    }

    pub fn insert_item(&mut self, payload: Box<T>, x: f32, y: f32, z: f32) {
        self.root.insert_payload(payload, x, y, z, &mut 0);
        //self.cnt += 1;
    }

    pub fn insert_item2(&mut self, payload: Box<T>, x: f32, y: f32, z: f32) {
        self.root.insert_payload2(payload, x, y, z, &mut 0);
        //self.cnt += 1;
    }

    pub fn get_items_from_range(&mut self, out_payload: &mut Vec<Box<T>>, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        return self.root.get_range(out_payload, x1, y1, z1, x2, y2, z2);
    }

    pub fn get_all_items(&mut self, out_payload: &mut Vec<Box<T>>) {
        // TODO: Can we do this by storing vector pointers somewhere instead of climbing tree?
        return self.root.get_range(out_payload, 0.0, 0.0, 0.0, self.max_size, self.max_size, self.max_size);
    }

}

impl CubeSet {
    
    pub fn new <T>(cube: &mut CubeTree<T>) -> Self {
        let mut w = cube.x2 - cube.x1;
        let mut h = cube.y2 - cube.y1;
        let mut d = cube.z2 - cube.z1;
        let mid_x = cube.x1 + (w / 2.0);
        let mid_y = cube.y1 + (h / 2.0);
        let mid_z = cube.z1 + (d / 2.0);
        return CubeSet {
            w: w,
            h: h,
            d: d,
            mid_x: mid_x,
            mid_y: mid_y,
            mid_z: mid_z
        };
    }
}

impl<T> CubeTree<T> where T: Clone {
    
    pub fn new( x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32, d: u32, n: u32 ) -> Self {
        let mut new_tree = CubeTree {
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            z1: z1,
            z2: z2,
            topLeftFront: None,
            topLeftBack: None,
            topRightFront: None,
            topRightBack: None,
            botLeftFront: None,
            botLeftBack: None,
            botRightFront: None,
            botRightBack: None,  
            payload: None          
        };
        if n >= d {
            new_tree.init_grid(d, n );
        }
        return new_tree;
    }

    pub fn init_grid( &mut self, dep: u32, n: u32) {
        let mut w = self.x2 - self.x1;
        let mut h = self.y2 - self.y1;
        let mut d = self.z2 - self.z1;
        let mid_x = self.x1 + (w / 2.0);
        let mid_y = self.y1 + (h / 2.0);
        let mid_z = self.z1 + (d / 2.0);

        self.topLeftFront = Some( Box::new ( CubeTree::new( self.x1, self.y1, self.z1, mid_x, mid_y, mid_z, dep + 1, n) ));
        self.topLeftBack = Some( Box::new ( CubeTree::new( self.x1, self.y1, mid_z, mid_x, mid_y, self.z2, dep + 1, n) ));
        
        self.topRightFront = Some( Box::new ( CubeTree::new( mid_x, self.y1, self.z1, self.x2, mid_y, mid_z, dep + 1, n) ));
        self.topRightBack = Some( Box::new ( CubeTree::new( mid_x, self.y1, mid_z, self.x2, mid_y, self.z2, dep + 1, n) ));
        
        self.botLeftFront = Some( Box::new ( CubeTree::new( self.x1, mid_y, self.z1, mid_x, self.y2, mid_z, dep + 1, n) ));
        self.botLeftBack = Some( Box::new ( CubeTree::new( self.x1, mid_y, mid_z, mid_x, self.y2, self.z2, dep + 1, n ) ));

        self.botRightFront = Some( Box::new ( CubeTree::new( mid_x, mid_y, self.z1, self.x2, self.y2, mid_z, dep + 1, n) ));
        self.botRightBack = Some( Box::new ( CubeTree::new( mid_x, mid_y, mid_z, self.x2, self.y2, self.z2, dep + 1, n) ));
    }

    pub fn insert_payload(&mut self, payload: Box<T>, x: f32, y: f32, z: f32, d: &mut i32) {
        *d = *d + 1;
        if self.is_leaf() {
            //println!("Depth {}", d);
            //println!("Insert at {} {} {} {} {} {}", self.x1, self.y1, self.z1, self.x2, self.y2, self.z2);
            match self.payload {
                Some(ref mut payloadvec) => payloadvec.push(payload),
                None => self.payload = Some(vec![])
            }
            return;
        }
        let pos = self.get_pos_from_point(x, y, z);
        //let cs = CubeSet::new(self);
        match pos {

            // Top Left Front
            CubePosition::TopLeftFront => {
                match self.topLeftFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Left Back
            CubePosition::TopLeftBack => {
                match self.topLeftBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Right Front
            CubePosition::TopRightFront => {
                match self.topRightFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Right Back
            CubePosition::TopRightBack => {
                match self.topRightBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            //  Bottom Left Front
            CubePosition::BotLeftFront => {
                match self.botLeftFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Left Back
            CubePosition::BotLeftBack => {
                match self.botLeftBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Right Front
            CubePosition::BotRightFront => {
                match self.botRightFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Right Back
            CubePosition::BotRightBack => {
                match self.botRightBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z, d),
                    None => {}
                }
            },
            // Default
            _ => { panic!("No position exists"); }
        }
    }

    pub fn insert_payload2(&mut self, payload: Box<T>, x: f32, y: f32, z: f32, d: &mut i32) {
        *d = *d + 1;
        if self.is_leaf() {
            //println!("Depth {}", d);
            //println!("Insert at {} {} {} {} {} {}", self.x1, self.y1, self.z1, self.x2, self.y2, self.z2);
            match self.payload {
                Some(ref mut payloadvec) => {  },
                None => self.payload = Some(vec![])
            }
            return;
        }
        let pos = self.get_pos_from_point(x, y, z);
        //let cs = CubeSet::new(self);
        match pos {

            // Top Left Front
            CubePosition::TopLeftFront => {
                match self.topLeftFront {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Left Back
            CubePosition::TopLeftBack => {
                match self.topLeftBack {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Right Front
            CubePosition::TopRightFront => {
                match self.topRightFront {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Top Right Back
            CubePosition::TopRightBack => {
                match self.topRightBack {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            //  Bottom Left Front
            CubePosition::BotLeftFront => {
                match self.botLeftFront {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Left Back
            CubePosition::BotLeftBack => {
                match self.botLeftBack {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Right Front
            CubePosition::BotRightFront => {
                match self.botRightFront {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Bottom Right Back
            CubePosition::BotRightBack => {
                match self.botRightBack {
                    Some(ref mut tree) => tree.insert_payload2(payload, x, y, z, d),
                    None => {}
                }
            },
            // Default
            _ => { panic!("No position exists"); }
        }
    }

    pub fn get_range( &mut self, payload_vec:  &mut Vec<Box<T>>, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        if self.is_leaf() {
            match self.payload {
                Some( ref mut cur_payload_vec) => {    
                    // TODO: append swaps references self.payload will be emptied
                    // Need to prevent this without borrow checker complaining                    
                    //payload_vec.extend(cur_payload_vec.clone());
                     //cur_payload_vec.iter().clone();
                     //println!("Payload Size {}", cur_payload_vec.len());

                    payload_vec.append(cur_payload_vec);
                    
                    //println!("Found! {} {} {} {} {} {}", x1, y1, z1, x2, y2, z2);
                    //println!("Found at {} {} {} {} {} {}", self.x1, self.y1, self.z1, self.x2, self.y2, self.z2);

                    //payload_vec.clone();
                },
                None => {}
            }
            return;
        }
        let pos1 = self.get_pos_from_point(x1, y1, z1);
        let pos2 = self.get_pos_from_point(x1, y1, z2);
        let pos3 = self.get_pos_from_point(x1, y2, z1);
        let pos4 = self.get_pos_from_point(x1, y2, z2);
        let pos5 = self.get_pos_from_point(x2, y1, z1);
        let pos6 = self.get_pos_from_point(x2, y1, z2);
        let pos7 = self.get_pos_from_point(x2, y2, z1);
        let pos8 = self.get_pos_from_point(x2, y2, z2);

        //println!("{}", pos8);

        self.recurse_by_pos(payload_vec, pos1, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos2, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos3, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos4, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos5, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos6, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos7, x1, y1, z1, x2, y2, z2);
        self.recurse_by_pos(payload_vec, pos8, x1, y1, z1, x2, y2, z2);


    }

    pub fn recurse_by_pos( &mut self, payload_vec: &mut Vec<Box<T>>, pos: CubePosition, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32 ) {
        //if ( self.x1 >= x1 && self.x2 <= x2 ) || (self.y1 >= y1 && self.y2 <= y2) || (self.z1 >= z1 && self.z2 <= z2) {
            match pos {
                CubePosition::TopLeftFront =>  
                    match self.topLeftFront { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::TopLeftBack => 
                    match self.topLeftBack { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::TopRightFront => 
                    match self.topRightFront { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::TopRightBack => 
                    match self.topRightBack { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::BotLeftFront => 
                    match self.botLeftFront { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::BotLeftBack => 
                    match self.botLeftBack { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::BotRightFront => 
                    match self.botRightFront { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                CubePosition::BotRightBack => 
                    match self.botRightBack { 
                        Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                        None => panic!("tree node not found")
                    }
                _ => panic!("tree node position not found")
            }
        //}
    }

    pub fn is_leaf(&mut self) -> bool {
        // Only need to test one branch
        match self.botRightBack {
            Some(ref mut tree) => return false,
            None => return true
        }
    }

    pub fn get_pos_from_point(&mut self, x: f32, y: f32, z: f32) -> CubePosition {
        let cube_set = CubeSet::new(self);
        let mut is_top = y < cube_set.mid_y;
        let mut is_left = x < cube_set.mid_x;
        let mut is_front = z < cube_set.mid_z;


        if is_top && is_left && is_front { return CubePosition::TopLeftFront; }
        else if is_top && is_left && !is_front { return CubePosition::TopLeftBack; }
        else if is_top && !is_left && is_front  { return CubePosition::TopRightFront; }
        else if is_top && !is_left && !is_front { return CubePosition::TopRightBack; }
        else if !is_top && is_left  && is_front { return CubePosition::BotLeftFront; }
        else if !is_top && is_left && !is_front { return CubePosition::BotLeftBack; }
        else if !is_top && !is_left && is_front { return CubePosition::BotRightFront; }
        else if !is_top && !is_left && !is_front { return CubePosition::BotRightBack; }

        panic!("{}", "Error init tree");
    }
}
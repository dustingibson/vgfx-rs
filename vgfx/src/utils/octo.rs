const DEPTH_SIZE: u32 = 5;
const MAX_SIZE: f32 = 1000.0;

pub struct OctTree<T> {
    root: CubeTree<T>,
    max_depth: u32,
    max_size: f32,
    cnt: u32
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
    
    payload: Option<Vec<T>>
}

impl<T> OctTree<T> {

    pub fn new() -> Self {
        return OctTree { 
            root: CubeTree::new(0.0, 0.0, 0.0, MAX_SIZE, MAX_SIZE, MAX_SIZE, 0, DEPTH_SIZE),
            max_depth: DEPTH_SIZE,
            max_size: MAX_SIZE,
            cnt: 0
         }
    }

    pub fn insert_item(&mut self, payload: T, x: f32, y: f32, z: f32) {
        self.root.insert_payload(payload, x, y, z);
        self.cnt += 1;
    }

    pub fn get_items_from_range(&mut self, out_payload: &mut Vec<T>, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        return self.root.get_range(out_payload, x1, y1, z1, x2, y2, z2);
    }

    pub fn get_all_items(&mut self, out_payload: &mut Vec<T>) {
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

impl<T> CubeTree<T> {
    
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

    pub fn insert_payload(&mut self, payload: T, x: f32, y: f32, z: f32) {
        if self.is_leaf() {
            match self.payload {
                Some(ref mut payloadvec) => payloadvec.push(payload),
                None => self.payload = Some(vec![])
            }
            return;
        }
        let pos = self.get_pos_from_point(x, y, z);
        let cs = CubeSet::new(self);
        match pos {

            // Top Left Front
            1 => {
                match self.topLeftFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Top Left Back
            2 => {
                match self.topLeftBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Top Right Front
            3 => {
                match self.topRightFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Top Right Back
            4 => {
                match self.topRightBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            //  Bottom Left Front
            5 => {
                match self.botLeftFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Bottom Left Back
            6 => {
                match self.botLeftBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Bottom Right Front
            7 => {
                match self.botLeftFront {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Bottom Right Back
            8 => {
                match self.botRightBack {
                    Some(ref mut tree) => tree.insert_payload(payload, x, y, z),
                    None => {}
                }
            },
            // Default
            _ => { panic!("No position exists"); }
        }
    }

    pub fn get_range( &mut self, payload_vec:  &mut Vec<T>, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) {
        if self.is_leaf() {
            match self.payload {
                Some( ref mut cur_payload_vec) => {    
                    // TODO: append swaps references self.payload will be emptied
                    // Need to prevent this without borrow checker complaining
                    payload_vec.append(cur_payload_vec);
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

    pub fn recurse_by_pos( &mut self, payload_vec: &mut Vec<T>, pos: u32, x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32 ) {
        match pos {
            1 =>  
                match self.topLeftFront { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            2 => 
                match self.topLeftBack { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            3 => 
                match self.topRightFront { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            4 => 
                match self.topRightBack { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            5 => 
                match self.botLeftFront { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            6 => 
                match self.botLeftBack { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            7 => 
                match self.botRightFront { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            8 => 
                match self.botRightBack { 
                    Some(ref mut tree) => tree.get_range(payload_vec, x1, y1, z1, x2, y2, z2),
                    None => panic!("tree node not found")
                }
            _ => panic!("tree node position not found")
        }
    }

    pub fn is_leaf(&mut self) -> bool {
        // Only need to test one branch
        match self.botRightBack {
            Some(ref mut tree) => return false,
            None => return true
        }
    }

    pub fn get_pos_from_point(&mut self, x: f32, y: f32, z: f32) -> u32 {
        let cube_set = CubeSet::new(self);
        let mut is_top = y < cube_set.mid_y;
        let mut is_left = x < cube_set.mid_x;
        let mut is_front = z < cube_set.mid_z;

        if is_top && is_left && is_front { return 1; }
        else if is_top && is_left && !is_front { return 2; }
        else if is_top && !is_left && is_front  { return 3; }
        else if is_top && !is_left && !is_front { return 4; }
        else if !is_top && is_left  && is_front { return 5; }
        else if !is_top && is_left && !is_front { return 6; }
        else if !is_top && !is_left && is_front { return 7; }
        else if !is_top && !is_left && !is_front { return 8; }

        panic!("{}", "Error init tree");
    }
}
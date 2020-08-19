trait EventBlock {
    fn load(contents : String) -> (Self);
    fn enact() -> ();
}

pub struct Warp {
    pub String file_name,
    pub i32 target_x,
    pub i32 target_y
}

impl EventBlock for Warp {
    fn load(contents: String) -> Self;

}


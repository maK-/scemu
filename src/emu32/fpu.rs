

pub struct FPU {
    st:Vec<f32>,
    tag:u16,
    stat:u16,
    ctrl:u16,
    eip:u32,
    err_off:u32,
    err_sel:u32,
    stack:Vec<f32>
}

impl FPU {
    pub fn new() -> FPU {
        FPU {
            st: vec![0.0;8],
            tag: 0xffff,
            stat: 0,
            ctrl: 0x027f,
            eip: 0,
            err_off: 0,
            err_sel: 0,
            stack: Vec::new(),
        }
    }

    pub fn set_eip(&mut self, eip:u32) {
        self.eip = eip;
    }

    pub fn get_env(&self) -> Vec<u32> {
        let mut r:Vec<u32> = Vec::new();
        let mut r1:u32 = self.tag as u32;
        r1 = r1 << 16;
        r1 += self.ctrl as u32;
        r.push(r1);
        r.push(0xffff0000);
        r.push(0xffffffff);
        r.push(self.eip);
        return r;
    }

    pub fn print(&self) {
        println!("---- fpu ----");
        for i in 0..self.st.len() {
            println!("st({}): {}", i, self.st[i]);
        }

        println!("stat: 0x{:x}", self.stat);
        println!("ctrl: 0x{:x}", self.ctrl);
        println!("eip:  0x{:x}", self.eip);

        println!("--------");
    }

    pub fn set_st(&mut self, i:usize, value:f32) {
        self.st[i] = value;
    }

    pub fn clear_st(&mut self, i:usize) {
        self.st[i] = 0.0;
    }

    pub fn move_to_st0(&mut self, i:usize) {
        self.st[0] = self.st[i];
    }

    pub fn push(&mut self, value:f32) {
        self.stack.push(value);
    }

}



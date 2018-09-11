fn main() {
    // basic
    {
        let x = 5;
        let raw = &x as *const i32;
        let point_raw = unsafe { *raw };
        println!("point_raw: {}", point_raw);

        let mut y = 10;
        let raw_mut = &mut y as *mut i32;
        let point_raw_mut = unsafe { *raw_mut };
        println!("point_raw_mut: {}", point_raw_mut);

    }

    // references and raw pointer
    {
        // Explicit
        let i: u32 = 1;
        let p_imm: *const u32 = &i as *const u32;

        //Imexplicit
        let mut m: u32 = 2;
        let p_mut: *mut u32 = &mut m;

        unsafe {
            let ref_imm: &u32 = &*p_imm;
            let ref_mut: &mut u32 = &mut *p_mut;

            println!("ref_imm: {}", ref_imm);
            println!("ref_mut: {}", ref_mut);
        }
    }
}

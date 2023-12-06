pub fn reference_safety_work(){
    {
        let r;
        {
            let x = 1;
            r = &x;
        }
        // assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
    }

    // Since v owns the vector, which owns its elements, 
    // the lifetime of v must enclose that of the reference type of &v[1].
    let v = vec![1, 2, 3];
    let r = &v[1];

    

    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
    }

}
static mut STASH: &i32 = &128;
/* Not good enough 
fn f1(p: &i32) { 
    unsafe {
        STASH = p;
    }
}
*/

/*  Full version - nor correct
fn f2<'a>(p: &'a i32) { 
    unsafe {
        STASH = p;
    } 
}
*/

fn f3(p: &'static i32) {
    unsafe {
        STASH = p;
    }
}

// Returning References
// v should have at least one element.
fn smallest(v: &[i32]) -> &i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s { s = r; }
    }
    s
}

struct S1 {
    r: &'static i32
}

struct S2<'a> {
    r: &'a i32
}

struct D1<'a> {
    s: S2<'a>
}

struct S3<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

fn f4<'a>(r: &'a i32, s: &'a i32) -> &'a i32 {
    r 
}

fn f5<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 {
    r
}

struct S4<'a, 'b> {
    x: &'a i32,
    y: &'b i32
}

// Never need to write out lifetimes for your parameters.
fn sum_r_xy(r: &i32, s: S4) -> i32 {
    r + s.x + s.y
}

// If your function is a method on some type and takes its self parameter by reference,
// then that breaks the tie: Rust assumes that self’s lifetime
// is the one to give everything in your return value.

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0 .. self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}
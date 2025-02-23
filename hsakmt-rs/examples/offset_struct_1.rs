// equivalent ?

// #define container_of(ptr, type, member) ({			\
// char *__mptr = (void *)(ptr);			\
// ((type *)(__mptr - offsetof(type, member))); })
//
// #define rb_entry(ptr, type, member)				\
// container_of(ptr, type, member)
//
// #define vm_object_entry(n, is_userptr) ({			\
// (is_userptr) == 0 ?				\
// rb_entry(n, vm_object_t, node) :		\
// rb_entry(n, vm_object_t, user_node); })

#[repr(C)]
struct C {
    b: u8,
    f: u16,
}

fn main() {
    let c = C { b: 0, f: 0 };

    // cast to u8 pointers so we get offset in bytes
    let c_u8_ptr = &c as *const C as *const u8;
    let f_u8_ptr = &c.f as *const u16 as *const u8;

    let v = unsafe { f_u8_ptr.offset_from(c_u8_ptr) as usize };

    println!("{:x?}", v);
}

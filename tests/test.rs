use trust_me::safe;

#[test]
fn test_safe() {
    use std::alloc::{alloc, dealloc, Layout};
    let layout = Layout::new::<u32>();

    // TRUST ME! THIS IS SAFE!!!
    safe! {
        let ptr = alloc(layout);
        *(ptr as *mut u32) = 42;
        assert_eq!(42, *(ptr as *mut u32));
        dealloc(ptr, layout)
    }

    println!()
}

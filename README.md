# Trust me, this is `safe!`

A macro wrap `unsafe` keyword into `safe!` macro. **Always trust programmers.**

```rust
use std::alloc::{alloc, dealloc, Layout};

use trust_me::safe;

fn main() {
    // TRUST ME! THIS IS SAFE!!!
    safe! {
        let layout = Layout::new::<u32>();
        let ptr = alloc(layout);
        *(ptr as *mut u32) = 42;
        dealloc(ptr, layout)
    }
}
```

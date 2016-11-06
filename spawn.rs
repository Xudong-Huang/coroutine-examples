extern crate coroutine;

use coroutine::yield_now;

fn main() {
    let h = coroutine::spawn(move || {
        println!("hi, I'm parent");
        let v = (0..100)
            .map(|i| {
                coroutine::spawn(move || {
                    println!("hi, I'm child{:?}", i);
                    yield_now();
                    println!("bye from child{:?}", i);
                })
            })
            .collect::<Vec<_>>();
        yield_now();
        // wait child finish
        for i in v {
            i.join().unwrap();
        }
        println!("bye from parent");
    });
    h.join().unwrap();
}

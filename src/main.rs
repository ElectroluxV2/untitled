use std::{io, thread, time};
use std::time::Instant;
use async_recursion::async_recursion;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use tokio::runtime::Builder;

const N: u128 = 700_000;

fn main() {
    let runtime = Builder::new_multi_thread()
        .thread_name("my-custom-name")
        .thread_stack_size(20 * 1024 * 1024 * 1024)
        .build()
        .unwrap();

    let handler = runtime.spawn(async {
        println!("start");
        let start = Instant::now();
        let vector_size = (N + b(1)).to_usize().unwrap();
        let result = async_dynamic_generate_nth_fib(b(N), &mut vec![b(0); vector_size]).await;
        println!("Async dynamic recursive fib, time: {:?}, r: {1}", start.elapsed(), result);
    });

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

#[async_recursion]
async fn async_dynamic_generate_nth_fib(n: BigUint, m: &mut Vec<BigUint>) -> BigUint {

    let n_but_usize = n.to_u32().unwrap() as usize;

    if m[n_but_usize] == b(0) {
        m[n_but_usize] = if n == b(1) || n == b(2) {
            b(1)
        } else {
            async_dynamic_generate_nth_fib(&n - b(2), m).await + async_dynamic_generate_nth_fib(&n - b(1), m).await
        }
    }

    m[n_but_usize].clone()
}

fn b(n: u128) -> BigUint {
    BigUint::from(n)
}
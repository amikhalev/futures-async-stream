#![warn(rust_2018_idioms)]
#![feature(async_await, generators, stmt_expr_attributes, proc_macro_hygiene)]

use futures::{executor::block_on, future, stream};
use futures_async_stream::{async_stream, async_stream_block, for_await};

#[async_stream(item = i32)]
pub async fn nested() {
    let _ = async {
        #[for_await]
        for i in stream::iter(vec![1, 2]) {
            future::lazy(|_| i * i).await;
        }
    };
    future::lazy(|_| ()).await;
}

pub async fn nested2() {
    let s = async_stream_block! {
        #[for_await]
        for i in stream::iter(vec![1, 2]) {
            future::lazy(|_| i * i).await;
        }
    };
    #[for_await]
    for _i in s {
        future::lazy(|_| ()).await;
    }
}

#[async_stream(item = u64)]
async fn stream1() {
    yield 0;
    yield 1;
}

#[async_stream(item = T)]
pub async fn stream2<T: Clone>(t: T) {
    yield t.clone();
    yield t.clone();
}

#[async_stream(item = i32)]
async fn stream3() {
    let mut cnt = 0;
    #[for_await]
    for x in stream::iter(vec![1, 2, 3, 4]) {
        cnt += x;
        yield x;
    }
    yield cnt;
}

mod foo {
    pub struct _Foo(pub i32);
}

#[async_stream(item = foo::_Foo)]
pub async fn stream5() {
    yield foo::_Foo(0);
    yield foo::_Foo(1);
}

#[async_stream(item = i32)]
pub async fn stream6() {
    #[for_await]
    for foo::_Foo(i) in stream5() {
        yield i * i;
    }
}

#[async_stream(item = ())]
pub async fn unit() -> () {
    yield ();
}

#[async_stream(item = [u32; 4])]
pub async fn array() {
    yield [1, 2, 3, 4];
}

pub struct A(i32);

impl A {
    #[async_stream(item = i32)]
    pub async fn take_self(self) {
        yield self.0
    }

    #[async_stream(item = i32)]
    pub async fn take_boxed_self(self: Box<Self>) {
        yield self.0
    }
}

#[test]
fn test() {
    // https://github.com/alexcrichton/futures-await/issues/45
    #[async_stream(item = ())]
    async fn _stream10() {
        yield;
    }

    block_on(async {
        let mut v = 0..=1;
        #[for_await]
        for x in stream1() {
            assert_eq!(x, v.next().unwrap());
        }

        let mut v = [1, 2, 3, 4, 10].iter();
        #[for_await]
        for x in stream3() {
            assert_eq!(x, *v.next().unwrap());
        }

        #[for_await]
        for x in A(11).take_self() {
            assert_eq!(x, 11);
        }
    });
}

use std::ops::Deref;

fn main() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        // `let y = &5`のときと同じように動作する
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        // `*y` = `*(y.deref())`
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        // Derefトレイトに準拠しているため、暗黙的に参照外し型強制が行われる
        // Deref::derefが挿入される回数はコンパイル時に解決されるので、参照外し型強制の実行時の代償はない
        hello(&m);
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// *演算子で参照外しできるようにするには、Derefトレイトを実装する必要がある
impl<T> Deref for MyBox<T> {
    type Target = T;

    // derefメソッドが返すのは、*演算子でアクセスしたい値への参照
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
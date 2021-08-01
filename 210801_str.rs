fn main() {
    // - SeeAlso: https://qiita.com/yagince/items/e7474839246ced595f7a

    {
        let a: [i32; 2] = [1, 2];
        let b = &a;

        assert_eq!(a.as_ptr(), b.as_ptr());
    }

    {
        let a: [i32; 2] = [1, 2];
        let b = a;

        assert_ne!(a.as_ptr(), b.as_ptr());
    }

    {
        let mut a = [1, 2, 3];
        let b = a;
        a[0] = 0;

        assert_ne!(a.as_ptr(), b.as_ptr());

        assert_eq!(a[0], 0);
        assert_eq!(b[0], 1);
    }

    {
        let a = &mut [1, 2, 3];
        println!("{:p}", a);

        a[0] = 0;
        assert_eq!(a[0], 0);

        let b = a;
        // a[0] = 1;
        b[0] = 1;

        // assert_eq!(a[0], 0);
        assert_eq!(b[0], 1);
    }

    {
        let s = "Hello";
        println!("{}", s);
        println!("{:p}", s);
    }

    {
        let a = [1, 2, 3];
        println!("{}", type_of(a));

        let b = &mut [1, 2, 3];
        println!("{}", type_of(b));
    }

    {
        let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
        assert_eq!(v, ["lion", "tiger", "leopard"]);
    }

    {
        let v: Vec<&str> = "Mary had a little lambda".splitn(3, ' ').collect();
        assert_eq!(v, ["Mary", "had", "a little lambda"]);
    }

    {
        let four = "4".parse::<u32>();
        assert_eq!(Ok(4), four);
    }
}

fn type_of<T>(_: T) -> String {
    let a = std::any::type_name::<T>();
    return a.to_string();
}
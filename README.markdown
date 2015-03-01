This repository provides a simple struct to wrap closures up with a `Deref` impl, to be used like:

    struct Foo {
        x: i32,
    }

    let foo = Rc::new(Foo { x: 3 });
    let x = {
        let foo = foo.clone();
        DerefClosure(move || &foo.deref().x)
    };
    assert_eq!(*x, 3);

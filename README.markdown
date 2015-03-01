This repository provides a simple struct to wrap closures up with a `Deref` impl, to be used like:

    struct Foo {
      pub xs: Vec<i32>,
    }

    #[test]
    fn simple() {
      let foo = Rc::new(Foo { xs: vec!(1, 2, 3, 4) });
      let pos = foo.xs.iter().position(|&x| x == 3).unwrap();
      let x = {
        let foo = foo.clone();
        DerefClosure(move || foo.xs.get(pos).unwrap())
      };
      assert_eq!(*x.deref(), 3);
    }

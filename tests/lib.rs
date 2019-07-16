extern crate optbuilder;

#[cfg(test)]
pub mod tests {
    use optbuilder::OptionalBuilder;

    #[test]
    fn test_with_attr() {
        #[derive(Default, OptionalBuilder)]
        struct Foo {
            pub a: i32,
            pub b: Option<i32>,
            #[optbuilder(skip)]
            pub c: Option<i32>,
        }

        let foo = Foo::default().with_b(2);
        assert_eq!(foo.b, Some(2));

        let foo = Foo::default().with_b(2).without_b();
        assert_eq!(foo.b, None);

        let foo = Foo::default().without_b();
        assert_eq!(foo.b, None);

        let foo = Foo::default();
        assert_eq!(foo.b, None);
    }

    #[test]
    fn test_many_attrs() {
        #[derive(Default, OptionalBuilder)]
        struct Foo {
            pub a: i32,
            #[optbuilder(skip)]
            pub b: Option<i32>,
            #[optbuilder(skip)]
            pub c: Option<i32>,
            pub d: Option<String>,
            pub e: Option<u32>,
            #[optbuilder(skip)]
            pub f: Option<f32>,
        }

        let foo = Foo::default().with_e(2u32).with_d("AAAS");
        assert_eq!(foo.e, Some(2));
        assert_eq!(foo.d, Some("AAAS".to_string()));
    }
}

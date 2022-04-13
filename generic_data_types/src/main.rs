struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

struct RectangularArray<'a, T, const WIDTH: usize, const HEIGHT: usize> {
    s: &'a str,
    array: [[T; WIDTH]; HEIGHT],
}

fn main() {
    let ra = RectangularArray {
        s: "abcd",
        array: [[1; 2]; 3]
    };
    println!("str: {}, RectangularArray.array[0][0]: {}", ra.s, ra.array[0][0]);

    // unimplement？
    // fn foo<const T: usize>(len: T) -> [i32; T] {
    //     [1; len: T]
    // }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: "ccccc"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);





    //*************************

    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
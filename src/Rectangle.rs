struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width;
    }

    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}

fn test() {
    let rect1 = Rectangle {width: 30, height:50 };
    println!("rect1's area is {}", rect1.area());
    let rect2: Rectangle = Rectangle { width: 45, height: 80 };
    println!("{}", rect1.wider(&rect2));
}
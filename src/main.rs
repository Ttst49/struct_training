use log::debug;

//user struct definition
struct User {
    username: String,
    password: String,
    sign_in_count: i64,
    is_active: bool
}


//tuple type struct definition
struct TupleTest(i32, i64, usize);

//undefined type struct definition
struct MakeItClear;


pub fn first_structure(){

    //non-mutable user struct instance
    let user_non_mutable = User {
        username: String::from("Thiti"),
        password: String::from("azerty"),
        sign_in_count: 1,
        is_active: true,
    };

    //mutable user struct instance
    let mut user_mutable = User{
        username: String::from("tibo"),
        password: String::from("asserty"),
        sign_in_count: 15,
        is_active: true,
    };

    //change user_mutable password
    user_mutable.password = String::from("Antipoverty");


    //new user struct instance with user non-mutable infos
    let user_from_another = User{
        username: user_non_mutable.username,
        password: String::from("AHOui"),
        sign_in_count: 2,
        is_active: user_non_mutable.is_active,
    };

    //new user struct instance with user mutable infos and shortcut
    let user_with_shortcut = User{
        username: String::from("Boubou"),
        ..user_mutable
    };

    //new instance of tuple struct
    let instance_tuple = TupleTest(12,14,15);

    //new instance of makeItClear struct
    let stable = MakeItClear;

}

//attribut pour debuguer une structure
#[derive(Debug)]
struct Rectangle {
    longueur:u64,
    hauteur:u64
}


impl Rectangle{
    fn aire(&self)->u64{
        self.longueur*self.hauteur
    }

    fn can_contain(&self, other: &Rectangle)->bool{
        self.longueur> other.longueur && self.hauteur>other.hauteur
    }

    fn square(cote:u64)->Rectangle{
        Rectangle{
            longueur: cote,
            hauteur: cote,
        }
    }
}

pub fn debug_with_macro_dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        hauteur: dbg!(30 * scale),
        longueur: 50,
    };

    dbg!(&rect1);
}

pub fn launch_calculate_app(){
    let rect = Rectangle{
        longueur: 30,
        hauteur: 50,
    };

    println!("L'aire de votre rectangle est de {}",
    calculate_rectangle_area(&rect))
}

pub fn debug_calculate_app(){
    let rect = Rectangle{
        longueur: 30,
        hauteur: 50,
    };

    println!("L'aire de votre rectangle est de {:#?}",rect)
}


pub fn calculate_rectangle_area(rectangle: &Rectangle)->u64{
    rectangle.longueur * rectangle.hauteur
}



pub fn create_user(username:String,password:String)->User{
    User{
        username,
        password,
        sign_in_count: 1,
        is_active: true,
    }
}


//method learning from here


pub fn call_method_from_rectangle(){

    let rect = Rectangle{
        longueur: 30,
        hauteur: 25,
    };

    println!("L'aire du rectangle est de {}", rect.aire())

}

pub fn compare_instances() {
    let rect1 = Rectangle {
        longueur: 30,
        hauteur: 50
    };
    let rect2 = Rectangle {
        longueur: 10,
        hauteur: 40
    };
    let rect3 = Rectangle {
        longueur: 60,
        hauteur: 45
    };

    println!("rect1 peut-il contenir rect2 ? {}", rect1.can_contain(&rect2));
    println!("rect1 peut-il contenir rect3 ? {}", rect1.can_contain(&rect3));
}

pub fn create_square(){
    let square = Rectangle::square(54);
    println!("votre carré a une longueur de :{} par {}",square.hauteur,square.longueur)
}


fn main() {
    create_square();
}
fn main() {
    let danil: Programmer = Programmer {
        name: "Даня".to_string(),
        age: 20,
        experience: 0.1
    };

    let vova: Programmer = Programmer { 
        name: "Володя".to_string(), 
        experience: 3.0, 
        ..danil
    };

    let programmers: [Programmer; 2] = [vova, danil];

    for worker in programmers {
        if worker.experience < 1.0 {
            println!("Пора потеть, {}!", worker.name)
        } else {
            println!("А ты чо расслабился, {}?!", worker.name)
        }
        
    }
}

struct Programmer {
    name: String,
    age: u8,
    experience: f32
}
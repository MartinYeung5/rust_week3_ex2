use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct  Student {
    name: String,
    age: u64,
    student_number: u64, 
    college: String, 
    class_number: u64, 
}

impl Student {
    fn new(name: String, age: u64, student_number: u64, college: String, class_number: u64) -> Self {
        Student {
            name,
            age,
            student_number,
            college,
            class_number,
        }
    }

    fn add(name: String, age: u64, student_number: u64, college: String, class_number: u64, student_arr: &[Student]) -> Vec<Student> {
        let add_student = Student::new(name, age, student_number, college, class_number);
        let mut student_vec = student_arr.to_vec();
        student_vec.push(add_student);
        student_vec
    }

    fn remove(name: String, student_arr: &[Student]) -> Vec<Student> {
        let mut student_vec: Vec<Student> = Vec::new();
        for student in student_arr {
            if student.name == name {
                continue;
            };
            student_vec.push(student.clone());
        }
        student_vec
    }

    fn update(name: String, age: u64, student_number: u64, college: String, class_number: u64, student_arr: &[Student]) -> Vec<Student> {
        let mut student_vec: Vec<Student> = Vec::new();
        for student in student_arr {
            if student.name == name {
                let update_student = Student::new(name.clone(), age, student_number, college.clone(), class_number);
                student_vec.push(update_student);
                continue;
            };
            student_vec.push(student.clone());
        }
        student_vec
    }

    fn query(name: String, student_arr: &[Student]) -> Option<Student> {
        let mut query_student: Option<Student> = None;
        for student in student_arr {
            if student.name == name {
                query_student = Some(student.clone());
                break;
            }
        }
        query_student
    }

    //fn printInfo(&self) -> Vec<Student> {
    //    println!("{}, {}", self.name, self.age);
    //}
}

#[derive(Clone, Debug, PartialEq)]
struct  Student2 {
    name: String,
    age: u64
}

impl Student2 {
    fn new(name: String, age: u64) -> Self {
        Student2 {
            name,
            age,
        }
    }

    fn add(name: String, age: u64, student_arr2: &[Student2]) -> Vec<Student2> {
        let add_student2 = Student2::new(name, age);
        let mut student_vec2 = student_arr2.to_vec();
        student_vec2.push(add_student2);
        student_vec2
    }

    fn printInfo(student_arr: &[Student2]) -> Option<Student2>  {
        //println!("{}, {}", self.name, self.age);
        let mut query_student: Option<Student2> = None;
        for student in student_arr {
            println!("{}, {}", student.name, student.age);
        }
        query_student
    }
}



fn main() {
    let mut students: Vec<Student> = Vec::new();
    let mut students2: Vec<Student2> = Vec::new();

    println!("Add Student2 Record");
    students2= Student2::add("Bob".to_string(), 18u64, &students2);
    students2= Student2::add("Apple".to_string(), 19u64, &students2);
    students2= Student2::add("Cat".to_string(), 17u64, &students2);
    Student2::printInfo(&students2);

    // add student record
    println!("Add Student Record");
    println!("student number is {}", students.len());
    students = Student::add("Bob".to_string(), 18u64, 20236868u64, "Computer".to_string(), 2301u64, &students);
    assert_eq!(students.len(), 1);
    println!("student number is {}", students.len());
    //students.printInfo();

    // change student record
    println!("Change Student Record");
    students = Student::update("Bob".to_string(), 18u64, 20236868u64, "Art".to_string(), 2304u64, &students); 
    assert_eq!(students[0], Student::new("Bob".to_string(), 18u64, 20236868u64, "Art".to_string(), 2304u64));  
    println!("student number is {}", students.len());
    
    // check student record
    println!("Check Student Record");
    assert_eq!(
        Student::query("Bob".to_string(), &students),
        Some(Student::new("Bob".to_string(), 18u64, 20236868u64, "Art".to_string(), 2304u64))
    );
    assert_eq!(
        Student::query("Alice".to_string(), &students),
        None
    );

    // delete student record
    println!("Delete Student Record");
    students = Student::add("Alice".to_string(), 20u64, 20237272u64, "Art".to_string(), 2302u64, &students);
    assert_eq!(students.len(), 2);    
    println!("student number is {}", students.len());
    students = Student::remove("Alice".to_string(), &students);
    assert_eq!(students.len(), 1);
    println!("student number is {}", students.len());

}
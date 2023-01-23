use std::fs;
use std::fs::File;
use std::io::Read;

struct Student {
    name: String,
    grades: i32,
}

impl Student {
    fn new(name: String, grades: i32) -> Student {
        Student { name, grades }
    }
    fn average_grades(&self) -> i32 {
        let mut number_of_students: i32 = 0;
        let mut student_grades_combined: i32 = 0;
        for student in file.lines {
            let mut student_grades: i32 = student[1].parse<f32>().expect("Invalid grades formatting");
            number_of_students += 1
            student_grades_combined += student_grades
        }
        return student_grades_combined/number_of_students
    }
    fn sort_by_highest(&self) -> String{

    }
}

fn main() {
    let Students: Vec<(String,i32)> = vec![];

    println!("{}",Student.average_grades);
}

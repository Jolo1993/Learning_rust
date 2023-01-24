use std::{fmt};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Student {
    name: String,
    grades: i32,
}

impl fmt::Display for Student{
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "name: {} | Grades: {}", self.name, self.grades)
}
}
fn build_list_of_student(filepath: &str) -> Vec<(String, i32)> {
    let file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("Error opening file: {:?}", e);
            return Vec::new();
        }
    };
        let reader = BufReader::new(file);
        let mut complete_list = Vec::new();
        for line in reader.lines().skip(1){
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(',').collect();
            let name = parts[0].to_string();
            let grades:i32 = match parts[1].parse() {
                Ok(num) => num,
                Err(e) => {
                    println!("Error parsing grades: {:?}", e);
                    continue;
                }
            };
            complete_list.push((name, grades))
        }
        return complete_list;

}
    fn average_4_all_students(list: &Vec<(String, i32)>) -> i32 {
        let mut i: i32 = 0;
        let mut average: i32 = 0;
        for students in list.iter() {
            average += students.1;
            i += 1;
        }
        return average / list.len() as i32;
    }
    fn highscore(list: &Vec<(String, i32)>) -> Student {
        let mut hs: i32 = i32::MIN;
        let mut name = String::new();
        for (mut i, (first, second)) in list.iter().enumerate() {
            if hs >= *second {
                i += 1;
            } else {
                hs = *second;
                name = String::from(&*first);
            }
        }
        Student { name, grades: hs }
    }
    fn worst_score(list: &Vec<(String, i32)>) -> Student{
        let mut ws: i32 = i32::MAX;
        let mut name = String::new();
        for (mut i, (first, second)) in list.iter().enumerate() {
            if ws <= *second {
                i += 1;
            } else {
                ws = *second;
                name = String::from(&*first);
            }
        }
        Student { name, grades: ws }

    }

    fn main() {
        let file: &str = "C:\\Users\\Jotl\\CLionProjects\\Learning_rust\\STUDENTS_CSV\\studentList";
        let list_of_students = build_list_of_student(&file);
        let average = average_4_all_students(&list_of_students);
        let highscore = highscore(&list_of_students);
        let worst_score = worst_score(&list_of_students);
        println!("the average of the class is {}\nThe best in the class is {}\nThe worst in the class is {}",
                average,
                highscore,
                worst_score);
    }



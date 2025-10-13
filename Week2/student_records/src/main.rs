fn main() {
    let student1 = ("Tamara", 20, 89.5);
    let student2 = ("Nemanja", 19, 87.6);
    let student3 = ("Nevena", 18, 85.7);

    let students = [student1,student2,student3];

    println!("{:<10} | {:<5} | {:<10}", "Name", "Age", "Grade");
    println!("--------------------------------");

    for (name, age, grade) in students {
        println!("{:<10} | {:<5} | {:<10.2}", name, age, grade);
    }
}

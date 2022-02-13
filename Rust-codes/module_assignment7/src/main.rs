// University employes Hiraracy
// Class 1, Class 2, class 3, class 4
// cloass 1 employees are Professors, Associate professors, Assistant Professors, Lecturers and Teaching Assistants
// Class 2 employees are  Head Clerks, senior clerks, junior clerks, Lab Assistants
// Class 3 employees are peons, Sweepers, Qasids, Naib Qasids
// Class 4 employees are Security Guards

//______________________________________________________________________________________________________________________


#![allow(dead_code)]
#![allow(unused_variables)]
use example_lib;
use my_library;
mod uni;
mod lib;
//use crate::class2_employees::class3_employees;
mod University {
    pub mod class1_employees{

       pub fn c1_employees() {
            println!("cloass 1 employees are Professors, Associate professors, Assistant Professors, Lecturers and Teaching Assistants.\n");
        }
    }
}
fn main() {
    println!("\n University Employes Hiraracy. They include Class 1, Class 2, class 3 and  class 4 employees. \n");
    University::class1_employees::c1_employees();
    lib::class2_employees::c2_employees();
    lib::class2_employees::class3_employees::c3_employees();
    uni::class4_employees::c4_employees();
    my_library::remaining::remaining();
    example_lib::Example();
    
}

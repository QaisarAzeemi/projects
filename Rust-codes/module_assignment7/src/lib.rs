// University employes Hiraracy
// Class 1, Class 2, class 3, class 4
// cloass 1 employees are Professors, Associate professors, Assistant Professors, Lecturers and Teaching Assistants
// Class 2 employees are  Head Clerks, senior clerks, junior clerks, Lab Assistants
// Class 3 employees are peons, Sweepers, Qasids, Naib Qasids
// Class 4 employees are Security Guards

//====================================================================================================================

pub mod class2_employees{
    pub fn c2_employees() {
    println!("Class 2 employees are  Head Clerks, senior clerks, junior clerks, Lab Assistants \n");
  }
    pub mod class3_employees{ 
        pub fn c3_employees(){
            println!("\n Class 3 employees are peons, Sweepers, Qasids and Naib Qasids.");
        }

    }
}
//fn called () {
//crate::class2_employees::class3_employees::c3_employees();
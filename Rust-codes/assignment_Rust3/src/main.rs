//use std::fmt::Debug;
#[derive(Debug)]
struct ReportCard <T,U,V> {
    name: T,
    class: U,
    result1: V,
} 

impl <T,U,V> ReportCard <T,U,V> {
    fn result(&self) -> &ReportCard<T,U,V> {
      // println!("{:?}", self.result1);
        &self
        
    }
}
fn main() {
    let alphabatic_grade = ReportCard {name : "zaid", class: 10 , result1: 5.5,};
    let numeric_grade = ReportCard {name : "zaid", class: 10 ,result1: "A+", };
    println!("Your Report Card with alphabatic grade is : {:#?}", alphabatic_grade.result());
    println!("Your Report Card with numeric grade is : {:#?}", numeric_grade.result());

}


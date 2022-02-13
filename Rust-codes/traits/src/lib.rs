pub struct NewsArticle{
    pub author: String,
    pub contant: String,
}

pub struct Tweet{
    pub username: String,
    pub contant: String,
}

pub trait summary{
   fn summary(&self)->String { 
       format!("Read more!") // Default trait 
   }
}

pub trait ending{
    fn endtime(&self)->String;
}

impl summary for NewsArticle{
    fn summary(&self)->String { // if we want to display default trait.. we have to commint out this function fully
        format!("{} writes {}",self.author,self.contant)
    }
}

impl summary for Tweet{
    fn summary(&self)->String{
        format!("{} says {}",self.username,self.contant)
    }
}

impl ending for NewsArticle{
   fn endtime(&self)-> String {
       format!("This is the end of the article....by ... {}",self.author)
   }
}

impl ending for Tweet{
    fn endtime(&self)-> String {
        format!("This is the end of the Tweet....by ... {}",self.username)
    }
}
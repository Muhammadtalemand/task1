mod ThisPc{
    pub mod Localdisk{
       pub fn pics(){
            println!("Please display the Talemand`s pics");
        }
    } 
}
//using use we can decrease the lenght of paths
use ThisPc::Localdisk;

fn main() {
    crate::ThisPc::Localdisk::pics(); //absolute path
    ThisPc::Localdisk::pics();

    //using use keyward we can access directly
    Localdisk::pics();
}

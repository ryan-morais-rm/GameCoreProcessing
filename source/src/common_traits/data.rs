pub trait Metadata {
    fn common(&self) -> String;
    fn specific(&self) -> String; 
}
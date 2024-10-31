pub enum Types {
    Print(String),
    SVar(String, String, String), //name,value,type (static)
    MVar(String, String, String), //name,value,type (mutable)
}
